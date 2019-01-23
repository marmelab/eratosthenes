# Sieve of Eratosthenes

A JavaScript vs. WebAssembly Prime Number Generator Benchmark

![Website Preview](./eratoscreen.png)

## Rationale

[Eratosthenes](https://marmelab.com/eratosthenes/) is a small website build in one hackday at marmelab in order to answer the question: is Webassembly faster than plain JavaScript ?

To do so, we created bootstraped a [create-rust-webpack](https://github.com/rustwasm/rust-webpack-template#readme) project and benchmarked the computation of prime numbers with the [Sieve of Eratosthenes algorithm](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

## Getting Started

**Requirements**

-   [Node.js 10+](https://nodejs.org/en/)
-   [Rustup](https://rustup.rs/) (or Rust + Cargo)

**Run The App**

1. Install dependencies with `make install`

2. Run the app with `make run`

3. Go to http://localhost:8080

## License

Eratosthenes is licensed under the [MIT License](./LICENSE.md), courtesy of [marmelab](http://marmelab.com).
