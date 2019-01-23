#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::iter::Peekable;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

pub struct PrimesIter {
    counter: u32,
    primes: Vec<u32>,
}

impl PrimesIter {
    pub fn new() -> Self {
        PrimesIter {
            counter: 2,
            primes: Vec::new(),
        }
    }

    fn is_prime(&self, n: u32) -> bool {
        for prime in self.primes.iter().take_while(|&p| p * p <= n) {
            if n % prime == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimesIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = self.counter;
        while ! self.is_prime(counter) {
            counter += 1;
        }
        self.primes.push(counter);

        self.counter = counter + 1;
        Some(counter)
    }
}

#[wasm_bindgen]
pub extern fn primes_up_to(limit: u32) -> Vec<u32> {
    set_panic_hook();

    PrimesIter::new().take_while(|p| p <= &limit).collect()
}
