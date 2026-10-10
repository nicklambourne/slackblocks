#!/usr/bin/env python3
"""Enforce 95% handwritten line coverage; report generated code separately."""
import json
import sys
from pathlib import Path
import xml.etree.ElementTree as ET

ROOT=Path(__file__).resolve().parents[1]
generated={ROOT/p for p in json.loads((ROOT/'generated/files.json').read_text())}
totals={'handwritten':[0,0],'generated':[0,0]}
for file in ET.parse(sys.argv[1]).iter('file'):
    path=Path(file.attrib['name']).resolve()
    metrics=file.find('metrics')
    kind='generated' if path in generated else 'handwritten'
    totals[kind][0]+=int(metrics.attrib['coveredstatements'])
    totals[kind][1]+=int(metrics.attrib['statements'])
for kind,(covered,total) in totals.items():
    print(f'{kind}: {covered}/{total} executable lines ({100*covered/total if total else 0:.2f}%)')
covered,total=totals['handwritten']
if not total or covered/total<.95:raise SystemExit('Handwritten PHP coverage must be at least 95%')
