import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
from threading import Thread
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import time
import unittest

from outbox import Outbox
from slackblocks import Message, SectionBlock


def message():
    return Message("C_TEST", text="Release ready", blocks=[SectionBlock("Release ready", block_id="operation-release-42")])


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        self.server.requests.append(body)
        mode = self.server.modes.pop(0)
        if mode in ("success", "accepted_timeout", "accepted_disconnect", "malformed"):
            self.server.accepted.append(body)
        if mode == "accepted_timeout":
            time.sleep(0.1)
        if mode == "accepted_disconnect":
            self.close_connection = True
            return
        codes = {"rate": 429, "long_rate": 429, "invalid_rate": 429, "server": 503, "redirect302": 302, "redirect307": 307}
        self.send_response(codes.get(mode, 200))
        self.send_header("Retry-After", "500" if mode == "long_rate" else "invalid" if mode == "invalid_rate" else "1")
        self.send_header("Location", f"http://127.0.0.1:{self.server.server_port}/followed")
        self.end_headers()
        data = b'broken' if mode == "malformed" else json.dumps({"ok": mode != "api_error", "error": "invalid_blocks", "channel": "C_TEST", "ts": "123.456"}).encode()
        try:
            self.wfile.write(data)
        except (BrokenPipeError, ConnectionResetError):
            pass


class DeliveryTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR"))
        self.path = str(Path(self.directory.name) / "outbox.sqlite")
        self.outbox = Outbox(self.path)
        self.outbox.enqueue("event-1", "release-42", message())
        self.server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.server.requests, self.server.accepted, self.server.modes = [], [], []
        self.thread = Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.endpoint = f"http://127.0.0.1:{self.server.server_port}/chat.postMessage"

    def tearDown(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join()
        self.outbox.close()
        self.directory.cleanup()

    def deliver(self, **kwargs):
        return self.outbox.deliver("release-42", "synthetic-test-token", endpoint=self.endpoint, now=100, **kwargs)

    def counts(self, requests, accepted):
        self.assertEqual((len(self.server.requests), len(self.server.accepted)), (requests, accepted))
        print(json.dumps({"test": self._testMethodName, "case": getattr(self, "case", None), "http_attempts": requests, "accepted_writes": accepted}))

    def test_success_duplicate_event_and_normal_restart(self):
        self.server.modes = ["success"]
        self.assertEqual(self.deliver()["status"], "sent")
        self.outbox.close()
        self.outbox = Outbox(self.path)
        self.outbox.enqueue("event-1", "release-42", message())
        self.outbox.recover_after_stopped_worker()
        self.assertEqual(self.deliver()["status"], "sent")
        self.counts(1, 1)
        with self.assertRaises(ValueError):
            self.outbox.enqueue("event-1", "different-operation", message())
        with self.assertRaises(ValueError):
            self.outbox.enqueue("event-2", "release-42", Message("C_TEST", text="Different"))

    def test_each_ambiguous_or_api_failure_has_no_automatic_retry(self):
        modes = {"api_error": ("failed", 0), "malformed": ("uncertain", 1),
                 "server": ("uncertain", 0), "redirect302": ("uncertain", 0),
                 "redirect307": ("uncertain", 0), "accepted_timeout": ("uncertain", 1),
                 "accepted_disconnect": ("uncertain", 1)}
        for i, (mode, (status, accepted)) in enumerate(modes.items()):
            with self.subTest(mode=mode):
                self.case = mode
                operation = f"operation-{i}"
                self.outbox.enqueue(f"event-{i+2}", operation, message())
                self.server.modes = [mode]
                previous = (len(self.server.requests), len(self.server.accepted))
                for _ in range(2):
                    result = self.outbox.deliver(operation, "synthetic-test-token", endpoint=self.endpoint, timeout=0.02)
                    self.assertEqual(result["status"], status)
                self.counts(previous[0] + 1, previous[1] + accepted)
                self.assertEqual(result["attempts"], 1)

    def test_rate_limit_wait_and_exhaustion_counts(self):
        self.server.modes = ["rate", "rate", "rate"]
        self.assertEqual(self.deliver()["not_before"], 101)
        self.assertEqual(self.deliver()["attempts"], 1)  # Too early: no HTTP write.
        self.outbox.deliver("release-42", "test", endpoint=self.endpoint, now=101)
        result = self.outbox.deliver("release-42", "test", endpoint=self.endpoint, now=102)
        self.assertEqual((result["status"], result["reason"]), ("failed", "retry_exhausted"))
        self.deliver()
        self.counts(3, 0)

    def test_rate_limit_then_success(self):
        self.server.modes = ["rate", "success"]
        self.deliver()
        result = self.outbox.deliver("release-42", "test", endpoint=self.endpoint, now=101)
        self.assertEqual(result["status"], "sent")
        self.counts(2, 1)

    def test_invalid_or_excessive_retry_delay_is_bounded(self):
        for mode, status in [("long_rate", "deferred"), ("invalid_rate", "failed")]:
            self.outbox.enqueue(mode, mode, message())
            self.server.modes = [mode]
            for _ in range(2):
                result = self.outbox.deliver(mode, "test", endpoint=self.endpoint)
                self.assertEqual(result["status"], status)
        self.counts(2, 0)

    def test_connection_refused_is_retained_without_retry(self):
        with socket.socket() as closed:
            closed.bind(("127.0.0.1", 0))
            port = closed.getsockname()[1]
        result = self.outbox.deliver("release-42", "test", endpoint=f"http://127.0.0.1:{port}/", timeout=0.02)
        self.assertEqual(result["status"], "uncertain")
        self.assertEqual(self.deliver()["attempts"], 1)
        self.counts(0, 0)

    def test_crash_after_remote_acceptance_before_local_receipt_commit(self):
        self.server.modes = ["success"]
        self.outbox.close()
        code = "from outbox import Outbox; import os,sys; o=Outbox(sys.argv[1]); o.deliver('release-42','test',endpoint=sys.argv[2],after_accept=lambda:os._exit(73))"
        process = subprocess.run([sys.executable, "-c", code, self.path, self.endpoint], cwd=Path(__file__).parent, check=False)
        self.assertEqual(process.returncode, 73)
        self.outbox = Outbox(self.path)
        self.assertEqual(self.outbox.get("release-42")["status"], "sending")
        self.outbox.recover_after_stopped_worker()
        self.assertEqual(self.deliver()["status"], "uncertain")
        self.counts(1, 1)
        # Independent mock-server acceptance ledger supplies reconciliation proof.
        self.assertEqual(self.server.accepted[0]["blocks"][0]["block_id"], "operation-release-42")
        with self.assertRaises(ValueError):
            self.outbox.reconcile_accepted("release-42", "C_WRONG", "123.456", "mock-server receipt")
        result = self.outbox.reconcile_accepted("release-42", "C_TEST", "123.456", "mock-server acceptance ledger, record 1")
        self.assertEqual(result["status"], "sent")
        self.deliver()
        self.counts(1, 1)


if __name__ == "__main__":
    unittest.main()
