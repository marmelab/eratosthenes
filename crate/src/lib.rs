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

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Ératosthène"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;

    Ok(())
}

#[wasm_bindgen]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

pub struct Primes {
    primes: Vec<u32>,
    current: u32,
}

pub fn primes() -> Primes {
    Primes {
        primes: vec![],
        current: 2,
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for i in self.current..u32::max_value() {
            if self.primes.iter().all(|x| i % x != 0) {
                self.primes.push(i);
                self.current = i+1;
                return Some(i);
            }
        }

        panic!("Integer overflowed!")
    }
}

pub struct Factorize {
    n: u32,
    primes: Peekable<Primes>,
}

impl Iterator for Factorize {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match (self.n, self.primes.peek().cloned()) {
            (1, _) => None,

            (m, Some(p)) => {
                if m < p * p {
                    self.n = 1;
                    return Some(m);
                }

                let (q, r) = (m / p, m % p);

                if r == 0 {
                    self.n = q;
                    Some(p)
                } else {
                    self.primes.next();
                    self.next()
                }
            }

            (_, None) => {
                unreachable!()
            }
        }
    }
}

fn factorize(n: u32) -> Factorize {
    Factorize {
        n: n,
        primes: primes().peekable(),
    }
}

#[wasm_bindgen]
pub extern fn primers(n: u32) -> u32 {
    factorize(n).n
}
