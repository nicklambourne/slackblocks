#!/usr/bin/env python3
"""Exercise Maven's full signed bundle through a loopback-only Central endpoint."""

import hashlib
import json
import os
import shutil
import subprocess
import tempfile
import threading
import xml.etree.ElementTree as ET
import zipfile
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


class LocalCentral(BaseHTTPRequestHandler):
    """Acknowledge uploads locally; no registry credentials or network writes."""

    def do_POST(self):
        self.rfile.read(int(self.headers.get("Content-Length", "0")))
        if self.path.startswith("/api/v1/publisher/upload?"):
            body = b"00000000-0000-0000-0000-000000000001"
        elif self.path.startswith("/api/v1/publisher/status?"):
            body = json.dumps({"deploymentState": "PUBLISHED", "purls": []}).encode()
        else:
            self.send_error(404)
            return
        self.send_response(200)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *args):
        pass


def verify_bundle(archive, version):
    prefix = f"io/github/nicklambourne/slackblocks/{version}/"
    artifacts = [
        f"slackblocks-{version}{suffix}"
        for suffix in (".pom", ".jar", "-sources.jar", "-javadoc.jar")
    ]
    with zipfile.ZipFile(archive) as bundle:
        names = {name for name in bundle.namelist() if not name.endswith("/")}
        expected = {
            prefix + name + suffix
            for name in artifacts
            for suffix in ("", ".asc", ".md5", ".sha1", ".sha256", ".sha512")
        }
        # The plugin may also checksum detached signatures. No root metadata is allowed.
        optional = {
            prefix + name + ".asc." + algorithm
            for name in artifacts
            for algorithm in ("md5", "sha1", "sha256", "sha512")
        }
        if not expected <= names or names - expected - optional:
            raise ValueError(
                f"Invalid Central bundle: missing={expected - names}, "
                f"unexpected={names - expected - optional}"
            )
        for name in artifacts:
            data = bundle.read(prefix + name)
            for algorithm in ("md5", "sha1", "sha256", "sha512"):
                expected_digest = hashlib.new(algorithm, data).hexdigest()
                actual = bundle.read(prefix + name + "." + algorithm).decode().strip()
                if actual != expected_digest:
                    raise ValueError(f"Invalid {algorithm} checksum for {name}")


def main():
    root = Path(__file__).resolve().parents[2]
    scratch = None
    if Path("/Volumes/Repos").exists():
        for name in ("Repos", "Cache", "Scratch"):
            if not os.path.ismount("/Volumes/" + name):
                raise SystemExit(f"Required /Volumes/{name} is not mounted")
        scratch = "/Volumes/Scratch"
    with tempfile.TemporaryDirectory(prefix="slackblocks-central-", dir=scratch) as tmp:
        # Match the publisher's isolated launcher: Maven 3.10 is used for normal
        # builds, but Central plugin 0.11 still needs 3.9.16 to stage clean bundles.
        launcher = Path(tmp) / "release-maven"
        (launcher / ".mvn/wrapper").mkdir(parents=True)
        shutil.copy2(root / "java/mvnw", launcher / "mvnw")
        shutil.copy2(
            root / "java/.mvn/wrapper/release-maven-wrapper.properties",
            launcher / ".mvn/wrapper/maven-wrapper.properties",
        )
        settings = Path(tmp) / "settings.xml"
        settings.write_text(
            "<settings><servers><server><id>central</id>"
            "<username>local-test</username><password>local-test</password>"
            "</server></servers></settings>"
        )
        with ThreadingHTTPServer(("127.0.0.1", 0), LocalCentral) as server:
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                subprocess.run(
                    [
                        str(launcher / "mvnw"),
                        "-B",
                        "-ntp",
                        "-s",
                        str(settings),
                        "-Prelease",
                        # Every supported JDK compiles and runs tests in Java CI.
                        # This isolated lane verifies the signed production bundle.
                        "-Dmaven.test.skip=true",
                        f"-DcentralBaseUrl=http://127.0.0.1:{server.server_port}",
                        "clean",
                        "deploy",
                    ],
                    cwd=root / "java",
                    check=True,
                )
            finally:
                server.shutdown()
                thread.join()
    version = ET.parse(root / "java/pom.xml").getroot().findtext("{*}version")
    verify_bundle(root / "java/target/central-publishing/central-bundle.zip", version)
    print(f"Signed Central bundle layout and checksums passed for {version}; no registry upload")


if __name__ == "__main__":
    main()
