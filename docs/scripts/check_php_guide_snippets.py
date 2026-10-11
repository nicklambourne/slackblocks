#!/usr/bin/env python3
"""Execute literal PHP guide code and compare every block example with its JSON tab."""

import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
files = [ROOT / "README.md", ROOT / "php/README.md"] + sorted((ROOT / "docs/docs").rglob("*.mdx"))
sending = {f"php/integrations/examples/{name}.php" for name in ("jolicode", "curl", "laravel")}
seen = set()
cases = []
for file in files:
    if file.parent.name == "php" and "reference" in file.parts:
        continue
    source = file.read_text()
    for match in re.finditer(r'^```php(?: title="([^"]+)")?\n([\s\S]*?)\n```', source, re.M):
        title, code = match[1], match[2]
        assert code.startswith("<?php\n"), f"{file}: PHP snippets must be complete scripts"
        if title:
            assert title in sending and title not in seen, title
            assert code == (ROOT / title).read_text().rstrip(), f"Sending example drift: {title}"
            subprocess.run(["php", "-l", str(ROOT / title)], check=True, capture_output=True)
            seen.add(title)
            continue
        expected = None
        if file.name == "using_blocks.mdx":
            heading = list(re.finditer(r"^## (.+) Block$", source[: match.start()], re.M))[-1][1]
            end = source.find("\n## ", match.end())
            section = source[match.end() : end if end >= 0 else None]
            payload = re.search(r"```json\n([\s\S]*?)\n```", section)
            assert payload, f"{heading}: missing documented JSON"
            expected = json.loads(payload[1])
        cases.append((str(file.relative_to(ROOT)), code, expected))
assert seen == sending, "All sending examples must be present and syntax checked"
blocks = (ROOT / "docs/docs/usage/using_blocks.mdx").read_text()
assert sum(expected is not None for _, _, expected in cases) == len(
    re.findall(r"^## .+ Block$", blocks, re.M)
)
cases.append(
    (
        "section_hello.php",
        (ROOT / "docs/examples/php/section_hello.php").read_text(),
        json.loads((ROOT / "docs/examples/section_hello.json").read_text()),
    )
)
for label, code, expected in cases:
    # An exact stdin script lets the documented relative autoloader run unchanged.
    result = subprocess.run(
        ["php", "-d", "zend.assertions=1", "-d", "assert.exception=1"],
        input=code,
        text=True,
        cwd=ROOT / "php",
        capture_output=True,
    )
    assert result.returncode == 0, f"{label}: {result.stdout}{result.stderr}\n{code}"
    output = result.stdout
    if expected is not None:
        assert json.loads(output) == expected, f"{label}: PHP differs from documented JSON"
print(
    f"Executed {len(cases)} PHP README/guide/reusable examples; every block matches its JSON tab; {len(seen)} sending examples match tested integration source and parse successfully."
)
