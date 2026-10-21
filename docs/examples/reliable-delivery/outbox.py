"""Conservative, single-dispatcher SQLite outbox for chat.postMessage.

Never automatically retry a possibly accepted write. No authentication tokens
are persisted. This example does not provide exactly-once delivery.
"""

import http.client
import json
import sqlite3
import time
from urllib.parse import urlsplit


class Outbox:
    def __init__(self, path):
        self.db = sqlite3.connect(path)
        self.db.row_factory = sqlite3.Row
        self.db.execute("PRAGMA journal_mode=WAL")
        self.db.execute("PRAGMA synchronous=FULL")
        self.db.executescript("""
            CREATE TABLE IF NOT EXISTS operations (
                id TEXT PRIMARY KEY, body TEXT NOT NULL, status TEXT NOT NULL,
                attempts INTEGER NOT NULL DEFAULT 0, not_before REAL NOT NULL DEFAULT 0,
                reason TEXT, receipt TEXT);
            CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY, operation_id TEXT NOT NULL);
        """)

    def close(self):
        self.db.close()

    def get(self, operation_id):
        row = self.db.execute("SELECT * FROM operations WHERE id=?", (operation_id,)).fetchone()
        if row is None:
            raise KeyError(operation_id)
        return dict(row)

    def enqueue(self, event_id, operation_id, native_message):
        # Canonical encoding of native output; never construct/patch blocks here.
        body = json.dumps(native_message.to_dict(), sort_keys=True, separators=(",", ":"))
        with self.db:
            self.db.execute("BEGIN IMMEDIATE")
            existing = self.db.execute(
                "SELECT * FROM operations WHERE id=?", (operation_id,)
            ).fetchone()
            if existing and existing["body"] != body:
                raise ValueError("operation_id must retain its original payload")
            old_event = self.db.execute(
                "SELECT operation_id FROM events WHERE id=?", (event_id,)
            ).fetchone()
            if old_event and old_event[0] != operation_id:
                raise ValueError("event_id must retain its original operation")
            self.db.execute(
                "INSERT OR IGNORE INTO operations(id,body,status) VALUES(?,?,'queued')",
                (operation_id, body),
            )
            self.db.execute("INSERT OR IGNORE INTO events VALUES(?,?)", (event_id, operation_id))
        return self.get(operation_id)

    def recover_after_stopped_worker(self):
        # Only call once the prior dispatcher is known to have stopped.
        with self.db:
            self.db.execute(
                "UPDATE operations SET status='uncertain',reason='dispatcher_interrupted' WHERE status='sending'"
            )

    def _finish(self, operation_id, status, reason=None, receipt=None, not_before=0):
        with self.db:
            self.db.execute(
                "UPDATE operations SET status=?,reason=?,receipt=?,not_before=? WHERE id=?",
                (
                    status,
                    reason,
                    json.dumps(receipt) if receipt else None,
                    not_before,
                    operation_id,
                ),
            )
        return self.get(operation_id)

    def reconcile_accepted(self, operation_id, channel, timestamp, evidence):
        row = self.get(operation_id)
        if (
            row["status"] != "uncertain"
            or not evidence
            or not isinstance(timestamp, str)
            or not timestamp
        ):
            raise ValueError(
                "require an uncertain operation and independently verified receipt evidence"
            )
        if channel != json.loads(row["body"])["channel"]:
            raise ValueError("receipt channel differs from intended destination")
        return self._finish(
            operation_id,
            "sent",
            "reconciled",
            {"channel": channel, "ts": timestamp, "evidence": evidence},
        )

    def deliver(
        self,
        operation_id,
        token,
        *,
        endpoint="https://slack.com/api/chat.postMessage",
        timeout=5,
        max_attempts=3,
        max_delay=60,
        clock=time.time,
        after_accept=None,
    ):
        """One bounded attempt. A scheduler may call again after a persisted 429 delay.

        after_accept exists only for the crash-injection test; no retry library sits
        underneath this transport. Redirects are never followed.
        """
        if (
            type(max_attempts) is not int
            or not 1 <= max_attempts <= 5
            or not 0 < timeout <= 30
            or not 0 <= max_delay <= 3600
        ):
            raise ValueError("bounded attempts, timeout and delay required")
        url = urlsplit(endpoint)
        if url.username or url.password or url.fragment or url.query:
            raise ValueError("endpoint must not contain credentials, query or fragment")
        if not (
            url.scheme == "https"
            and url.hostname == "slack.com"
            and url.path == "/api/chat.postMessage"
        ):
            if not (url.scheme == "http" and url.hostname == "127.0.0.1"):
                raise ValueError("only Slack HTTPS or the loopback test endpoint is allowed")
        now = clock()
        row = self.get(operation_id)
        if row["status"] != "queued" or row["not_before"] > now:
            return row
        if row["attempts"] >= max_attempts:
            return self._finish(operation_id, "failed", "retry_exhausted")
        # Persist before making the write. A crash from here onward is ambiguous.
        with self.db:
            claimed = self.db.execute(
                "UPDATE operations SET status='sending',attempts=attempts+1 WHERE id=? AND status='queued'",
                (operation_id,),
            ).rowcount
        if not claimed:
            return self.get(operation_id)
        connection = (
            http.client.HTTPSConnection if url.scheme == "https" else http.client.HTTPConnection
        )(url.hostname, url.port, timeout=timeout)
        try:
            connection.request(
                "POST",
                url.path,
                body=row["body"].encode(),
                headers={
                    "Authorization": f"Bearer {token}",
                    "Content-Type": "application/json; charset=utf-8",
                },
            )
            response = connection.getresponse()
            if response.status == 429:
                retry_after = response.getheader("Retry-After", "")
                if not retry_after.isdecimal():
                    return self._finish(operation_id, "failed", "invalid_retry_after")
                delay = int(retry_after)
                # Retry-After starts when the response arrives, not when the request
                # began. The injected clock lets tests advance time during the request.
                not_before = clock() + delay
                if row["attempts"] + 1 >= max_attempts:
                    return self._finish(operation_id, "failed", "retry_exhausted")
                if delay > max_delay:
                    return self._finish(
                        operation_id, "deferred", "delay_budget_exceeded", not_before=not_before
                    )
                return self._finish(operation_id, "queued", "rate_limited", not_before=not_before)
            if response.status != 200:
                return self._finish(operation_id, "uncertain", f"http_{response.status}")
            data = response.read(1024 * 1024 + 1)
            if len(data) > 1024 * 1024:
                return self._finish(operation_id, "uncertain", "response_too_large")
            try:
                result = json.loads(data)
            except (ValueError, UnicodeError):
                return self._finish(operation_id, "uncertain", "malformed_json")
            if not isinstance(result, dict) or type(result.get("ok")) is not bool:
                return self._finish(operation_id, "uncertain", "invalid_api_response")
            if result["ok"] is False:
                return self._finish(operation_id, "failed", str(result.get("error", "api_error")))
            if (
                result.get("channel") != json.loads(row["body"])["channel"]
                or not isinstance(result.get("ts"), str)
                or not result["ts"]
            ):
                return self._finish(operation_id, "uncertain", "missing_or_mismatched_receipt")
            if after_accept:
                after_accept()
            return self._finish(
                operation_id, "sent", receipt={"channel": result["channel"], "ts": result["ts"]}
            )
        except (OSError, http.client.HTTPException):
            return self._finish(operation_id, "uncertain", "connection_or_timeout")
        finally:
            connection.close()
