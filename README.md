# subxt-scale-ink
Testing using common scale types between `ink` and `wasm32-unknown-unknown` builds. Ie. rust wasm frontend sharing types with ink project, using subxt to submit extrinsics.

Issue:
Building `web` package fails due to `common` having a dependency on ink via openbrush.

`cargo +nightly contract build --manifest-path ./contract/Cargo.toml`

Building contract works

`cargo +nightly build --target wasm32-unknown-unknown --manifest-path ./web/Cargo.toml`

Building web fails with
```
error[E0433]: failed to resolve: use of undeclared crate or module `ink_engine`
  --> /home/jack/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ink_env-4.2.0/src/engine/off_chain/impls.rs:45:5
   |
45 | use ink_engine::{
   |     ^^^^^^^^^^ use of undeclared crate or module `ink_engine`

error[E0433]: failed to resolve: use of undeclared crate or module `secp256k1`
   --> /home/jack/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ink_env-4.2.0/src/engine/off_chain/impls.rs:286:13
    |
286 |         use secp256k1::{
    |             ^^^^^^^^^ use of undeclared crate or module `secp256k1`
...
```

Tried adding ink, ink_engine, etc as a dependency to `web` but no luck
