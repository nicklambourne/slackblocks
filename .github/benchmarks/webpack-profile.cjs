// Measurement-only observer. Both policies retain Docusaurus filesystem caching.
const Module = require('node:module');
const fs = require('node:fs');
const path = require('node:path');
const load = Module._load;
const seen = new WeakSet();
Module._load = function(request, ...args) {
  const value = load.call(this, request, ...args);
  if (request === './compiler' && args[0]?.filename.includes('/@docusaurus/bundler/lib/') && !seen.has(value)) {
    seen.add(value);
    const compile = value.compile;
    value.compile = async function(options) {
      const start = performance.now();
      const result = await compile(options);
      const records = (result.stats || [result]).map(stats => ({
        name: stats.compilation.name,
        cache: stats.compilation.compiler.options.cache,
        statistics: stats.toJson({all: false, timings: true, modules: true, cachedModules: true, nestedModules: true, errors: true, warnings: true}),
      }));
      if (process.env.DOCS_BENCH_RESULTS) fs.writeFileSync(path.join(process.env.DOCS_BENCH_RESULTS, `webpack-${process.pid}.json`), JSON.stringify({elapsed_s:(performance.now()-start)/1000, records}));
      return result;
    };
  }
  return value;
};
