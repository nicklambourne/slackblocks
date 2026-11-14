"""Bounded, read-only registry waits. Pending availability is not release success."""

import datetime
import json
from pathlib import Path
import re
import time
import urllib.error
import urllib.request


class Pending(RuntimeError):
    """The exact version is not yet available through the consumer's registry."""


def save(path, receipt):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(receipt, indent=2) + "\n")
    temporary.replace(path)


def wait(probe, path, identity, *, timeout=900, interval=30, clock=time.monotonic, sleep=time.sleep):
    """Retry only explicitly classified availability failures; retain every observation."""
    if timeout < 0 or interval <= 0:
        raise ValueError("Invalid availability wait bounds")
    start = clock()
    receipt = {**identity, "state": "awaiting_availability", "passed": False, "observations": []}
    try:
        while True:
            try:
                result = probe()
            except Pending as error:
                receipt["observations"].append({"at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "detail": str(error)})
                receipt["elapsed_seconds"] = round(clock() - start, 3)
                save(path, receipt)
                if clock() - start >= timeout:
                    raise
                sleep(min(interval, max(0, timeout - (clock() - start))))
            else:
                # Availability is separate from successful compilation and native execution.
                receipt.update(state="available", elapsed_seconds=round(clock() - start, 3))
                save(path, receipt)
                return result
    except Pending:
        raise
    except Exception as error:
        receipt.update(state="failed", error=str(error))
        save(path, receipt)
        raise


def finish(path, **updates):
    receipt = json.loads(Path(path).read_text())
    receipt.update(updates)
    save(path, receipt)


def open_response(url):
    request = urllib.request.Request(url, headers={"User-Agent": "slackblocks-release-verifier"})
    try:
        return urllib.request.urlopen(request, timeout=30)
    except urllib.error.HTTPError as error:
        if error.code in (404, 429, 502, 503, 504):
            raise Pending(f"Registry HTTP {error.code}: {url}") from error
        raise


def json_response(url):
    with open_response(url) as response:
        return json.load(response)


def packagist(version, commit):
    """Read the normal Composer feed, expanding its inherited package metadata."""
    url = "https://repo.packagist.org/p2/nicklambourne/slackblocks.json"
    data = json_response(url)
    previous = {}
    for entry in data["packages"]["nicklambourne/slackblocks"]:
        package = {**previous, **entry}
        # Composer metadata-minifier encodes deleted values as the string __unset.
        package = {key: value for key, value in package.items() if value != "__unset"}
        previous = package
        if package["version"].removeprefix("v") != version:
            continue
        if package["source"]["reference"] != commit or package["dist"]["reference"] != commit:
            raise ValueError("Packagist version points to a different release commit")
        if package["source"]["url"].removesuffix(".git") != "https://github.com/nicklambourne/slackblocks":
            raise ValueError("Packagist source repository differs")
        return package
    raise Pending(f"Composer public metadata does not yet contain {version}")


def missing_nuget_version(output):
    errors = set(re.findall(r"error (NU\d+|MSB\d+|NETSDK\d+)", output))
    packages = set(re.findall(r"Unable to find package (\S+) with version", output))
    return errors == {"NU1102"} and packages == {"Slackblocks"}
