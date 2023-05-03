#![cfg_attr(not(feature = "std"), no_std)]

use common::{Bar, Id};

#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn main() {
    let to_be_used_for_subxt_contract_call = Bar {
        inner: Id::U32(123),
    };
}
