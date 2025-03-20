mod utils;

use wasm_bindgen::prelude::*;
use aptos_sdk::{
    types::{
        LocalAccount,
    },
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, perps-expert-advisor!");
}

#[wasm_bindgen]
pub fn print_account() {
    let mut alice = LocalAccount::generate(&mut rand::rngs::OsRng);
    alert(alice.address().to_hex_literal());
}