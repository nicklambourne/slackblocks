// Identical timing instrumentation in both variants. Stabilize only the default
// cache-busting hash input traversal; filesystem enumeration order is not content.
const Module = require('node:module');
const fs = require('node:fs');
const path = require('node:path');
const original = Module._load;
const wrapped = new WeakSet();
const metrics = { scan_s: 0, index_s: 0, language_s: 0, html_reads: 0 };
const read = fs.readFile;
fs.readFile = function(file, ...args) { if (String(file).endsWith('.html')) metrics.html_reads++; return read.call(this, file, ...args); };
const promises = require('node:fs/promises');
const readPromise = promises.readFile;
promises.readFile = function(file, ...args) { if (String(file).endsWith('.html')) metrics.html_reads++; return readPromise.call(this, file, ...args); };
Module._load = function(request, ...args) {
  const value = original.call(this, request, ...args);
  if (request === 'klaw-sync' && args[0]?.filename.endsWith('/getIndexHash.js')) {
    return function(...params) {
      return value(...params).sort((a, b) => a.path < b.path ? -1 : a.path > b.path ? 1 : 0);
    };
  }
  if (!value || (typeof value !== 'object' && typeof value !== 'function') || wrapped.has(value)) return value;
  for (const [name, metric, async] of [['scanDocuments', 'scan_s', true], ['buildIndex', 'index_s', false]]) {
    if (typeof value[name] !== 'function') continue;
    const fn = value[name];
    value[name] = async ? async function(...params) {
      const start = performance.now(); try { return await fn.apply(this, params); } finally { metrics[metric] += (performance.now() - start) / 1000; }
    } : function(...params) {
      const start = performance.now(); try { return fn.apply(this, params); } finally { metrics[metric] += (performance.now() - start) / 1000; }
    };
    wrapped.add(value);
  }
  if (String(request).endsWith('language-search-indexes.cjs') && typeof value === 'function') {
    return function(...params) {
      const plugin = value(...params); const build = plugin.postBuild;
      plugin.postBuild = async function(...data) { const start = performance.now(); try { return await build.apply(this, data); } finally { metrics.language_s += (performance.now() - start) / 1000; } };
      return plugin;
    };
  }
  return value;
};
process.on('exit', () => {
  if (metrics.scan_s && process.env.DOCS_BENCH_RESULTS) {
    fs.writeFileSync(path.join(process.env.DOCS_BENCH_RESULTS, `node-${process.pid}.json`), JSON.stringify({ ...metrics, argv: process.argv, max_rss_kib: process.resourceUsage().maxRSS }));
  }
});
