const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const root = process.argv[2];
const { canonicalIndex, searchResults } = require(path.join(root, 'docs/scripts/check_search_reuse.cjs'));
const stable = value => Array.isArray(value) ? value.map(stable) : value && typeof value === 'object' ? Object.fromEntries(Object.keys(value).sort().map(key => [key, stable(value[key])])) : value;
const hash = value => crypto.createHash('sha256').update(value).digest('hex');
const result = { files: {}, generated: {} };
const build = path.join(root, 'docs/build');
for (const name of fs.readdirSync(build, { recursive: true }).sort()) {
  const file = path.join(build, name);
  if (!fs.statSync(file).isFile()) continue;
  const raw = fs.readFileSync(file);
  const record = { bytes: raw.length, sha256: hash(raw) };
  if (/(?:^|\/)search-index(?:-[a-z]+)?\.json$/.test(name)) {
    const data = JSON.parse(raw);
    record.canonical = hash(JSON.stringify(stable(canonicalIndex(data))));
    record.queries = hash(JSON.stringify(stable(['shared', 'decimal', 'Python', 'Elixir', 'shared decimal', 'message*', '+shared -hidden'].map(q => searchResults(data, q)))));
    record.documents = data.map(x => x.documents.length);
  }
  result.files[name] = record;
}
for (const area of ['docs/docs/reference', 'docs/versioned_docs', 'docs/versioned_sidebars']) {
  for (const name of fs.readdirSync(path.join(root, area), { recursive: true }).sort()) {
    const file = path.join(root, area, name);
    if (fs.statSync(file).isFile()) result.generated[`${area}/${name}`] = hash(fs.readFileSync(file));
  }
}
fs.writeFileSync(process.argv[3], JSON.stringify(result));
