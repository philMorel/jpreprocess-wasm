# jpreprocess-wasm

WebAssembly bindings for [jpreprocess](https://github.com/jpreprocess/jpreprocess) with bundled NAIST-JDIC.

## Build

```bash
wasm-pack build --release --target web
```

## Usage

```js
import init, { JPreprocess } from "./pkg/jpreprocess_wasm.js";

await init();

const jp = new JPreprocess();
const result = jp.analyze("例", true);

console.log(result.nodes[0].read);
console.log(result.nodes[0].pronunciation);
```

## API

* `new JPreprocess()`
* `analyze(text, preprocess?)`
* `inspect(text)`
* `normalize(text)`
* `fullContext(text)`

This project is a thin WebAssembly wrapper around jpreprocess.

## License

See [LICENSE](LICENSE) and [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
