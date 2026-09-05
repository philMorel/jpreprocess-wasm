# Third-party licenses

This project bundles or links the following third-party software. Their
copyright and license notices apply in addition to this project's `LICENSE`.

## jpreprocess

The `jpreprocess` 0.15.0 family used by this project, including
`jpreprocess-njd` and the bundled dictionary crate, is licensed under the
BSD 3-Clause License.

Source: <https://github.com/jpreprocess/jpreprocess>

The upstream project records these notices:

* OpenJTalk — Copyright (c) 2008-2016 Nagoya Institute of Technology
  Department of Computer Science.
* Lindera — Copyright (c) 2019 by the project authors.
* Yada: Yet Another Double-Array.

## NAIST-JDIC

The bundled NAIST-JDIC data is distributed under the BSD 3-Clause License.

Source: <https://github.com/jpreprocess/naist-jdic>

Copyright notices from the dictionary distribution:

* Copyright (c) 2009, Nara Institute of Science and Technology, Japan.
* Copyright (c) 2011-2017, The UniDic Consortium.
* Copyright (c) 2008-2016 Nagoya Institute of Technology Department of
  Computer Science.

## Rust and WebAssembly dependencies

The versions resolved in `Cargo.lock` have these license expressions:

| Dependency | Version | License |
| --- | ---: | --- |
| `wasm-bindgen` | 0.2.127 | MIT OR Apache-2.0 |
| `js-sys` | 0.3.104 | MIT OR Apache-2.0 |
| `serde` | 1.0.229 | MIT OR Apache-2.0 |
| `serde-wasm-bindgen` | 0.6.5 | MIT |

The complete license texts for these dependencies are available from their
source distributions and project repositories:

* <https://github.com/rustwasm/wasm-bindgen>
* <https://github.com/serde-rs/serde>
* <https://github.com/RReverser/serde-wasm-bindgen>
* <https://www.apache.org/licenses/LICENSE-2.0>
* <https://opensource.org/license/mit/>

The transitive Rust dependencies are resolved by Cargo and retain the license
notices supplied by their respective packages.

## BSD 3-Clause License

Copyright notices above refer to the applicable upstream work.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.
3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
