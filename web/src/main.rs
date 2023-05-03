#![cfg_attr(not(feature = "std"), no_std)]

use wasm_bindgen::prelude::*;

use parity_scale_codec::alloc::vec;
use parity_scale_codec::alloc::vec::Vec;

use common::{Bar, Id};

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let to_be_used_for_subxt_contract_call = Bar {
            inner: Id::U32(123),
        };
    });
}
