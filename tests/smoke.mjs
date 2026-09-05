import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { JPreprocess } from '../pkg/jpreprocess_wasm.js';

const here = dirname(fileURLToPath(import.meta.url));
const wasmPath = resolve(here, '../pkg/jpreprocess_wasm_bg.wasm');
await init({ module_or_path: await readFile(wasmPath) });

const jp = new JPreprocess();

function first(text, preprocess = true) {
  return jp.analyze(text, preprocess).nodes[0];
}

assert.equal(first('例').read, 'レイ');
assert.equal(first('例').pronunciation, 'レー');
assert.equal(first('学校').read, 'ガッコウ');
assert.equal(first('学校').pronunciation, 'ガッコー');
assert.equal(first('大阪').read, 'オオサカ');
assert.equal(first('大阪').pronunciation, 'オーサカ');

const so = jp.analyze('そうそう', true).nodes;
assert.equal(so[0].surface, 'そう');
assert.equal(so[0].read, 'ソウ');
assert.equal(so[0].pronunciation, 'ソー');

const oneBefore = first('1組', false);
assert.equal(oneBefore.surface, '１');
assert.equal(oneBefore.read, 'イチ');
assert.equal(oneBefore.pronunciation, 'イチ');

const oneAfter = first('1組', true);
assert.equal(oneAfter.surface, '一');
assert.equal(oneAfter.read, 'イチ');
assert.equal(oneAfter.pronunciation, 'イッ');

const inspect = jp.inspect('例');
assert.equal(inspect.before[0].read, 'レイ');
assert.equal(inspect.after[0].pronunciation, 'レー');

assert.equal(jp.normalize('1組'), '１組');
assert.ok(jp.fullContext('例').length > 0);

console.log('jpreprocess-wasm smoke: ok');
