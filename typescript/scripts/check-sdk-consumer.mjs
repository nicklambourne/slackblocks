import { mkdtempSync, cpSync, readdirSync, rmSync, existsSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const scratch = mkdtempSync(join(existsSync('/Volumes/Scratch') ? '/Volumes/Scratch' : tmpdir(), 'slackblocks-sdk-'));
const run = (command, args, cwd = scratch) => execFileSync(command, args, { cwd, stdio: 'inherit' });
try {
  run('pnpm', ['pack', '--pack-destination', scratch], root);
  const archive = readdirSync(scratch).find(name => name.endsWith('.tgz'));
  if (!archive) throw new Error('Package archive was not produced');
  writeFileSync(join(scratch, 'package.json'), JSON.stringify({ private: true, type: 'module' }));
  run('npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', join(scratch, archive), '@slack/web-api@8.2.0', 'typescript@5.9.3', '@types/node@22']);
  cpSync(join(root, 'test/fixtures/sdk-consumer.mts'), join(scratch, 'consumer.mts'));
  run(join(scratch, 'node_modules/.bin/tsc'), ['--strict', '--exactOptionalPropertyTypes', '--noUncheckedIndexedAccess', '--module', 'NodeNext', '--moduleResolution', 'NodeNext', '--target', 'ES2022', '--skipLibCheck', 'consumer.mts']);
  run(process.execPath, ['consumer.mjs']);
} finally {
  rmSync(scratch, { recursive: true, force: true });
}
