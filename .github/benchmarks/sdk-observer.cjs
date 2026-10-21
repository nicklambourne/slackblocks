// Measurement-only receipt before the unchanged consumer removes its scratch directory.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const original = fs.rmSync;
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
fs.rmSync = function(target, options) {
  if (process.env.SDK_RECEIPT && path.basename(String(target)).startsWith('slackblocks-sdk-') && fs.existsSync(path.join(target, 'consumer.mjs'))) {
    const files = fs.readdirSync(target);
    const tar = files.find(name => name.endsWith('.tgz'));
    const dependencies = {};
    function scan(dir) {
      for (const entry of fs.readdirSync(dir, {withFileTypes:true})) {
        const file = path.join(dir,entry.name);
        if(entry.isDirectory()) scan(file);
        else if(entry.name === 'package.json') dependencies[path.relative(target,file)] = hash(fs.readFileSync(file));
      }
    }
    scan(path.join(target,'node_modules'));
    const lock = JSON.parse(fs.readFileSync(path.join(target,'package-lock.json'),'utf8').replaceAll(String(target),'SDK_SCRATCH'));
    // The random temporary directory determines npm's default root name only.
    delete lock.name;
    if(lock.packages['']) delete lock.packages[''].name;
    fs.writeFileSync(process.env.SDK_RECEIPT,JSON.stringify({lock,dependencies,consumer_js_sha256:hash(fs.readFileSync(path.join(target,'consumer.mjs'))),tar_sha256:hash(fs.readFileSync(path.join(target,tar)))}));
  }
  return original.call(this,target,options);
};
require('node:module').syncBuiltinESMExports();
