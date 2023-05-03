# subxt-scale-ink
Testing common scale types between `ink` and `wasm32-unknown-unknown` builds. Ie. rust wasm frontend sharing types with ink project, using subxt to submit extrinsics.

Issue:
Building `web` package fails due to `common` having a dependency on ink via openbrush.

`cargo +nightly contract build --manifest-path ./contract/Cargo.toml`

Building contract works

`cargo +nightly build --target wasm32-unknown-unknown --manifest-path ./web/Cargo.toml`

Building web fails
