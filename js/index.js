const primes_up_to = max => {
    var sieve = [],
        i,
        j,
        primes = [];
    for (i = 2; i <= max; ++i) {
        if (!sieve[i]) {
            // i has not been marked -- it is prime
            primes.push(i);
            for (j = i << 1; j <= max; j += i) {
                sieve[j] = true;
            }
        }
    }
    return primes;
};

import('../crate/pkg').then(module => {
    console.log('Starting');
    const t0Rust = performance.now();
    const primesWithRust = module.primes_up_to(10000000);
    const t1Rust = performance.now();
    console.log(`Dernier nombre premier jusqu'à 10000000 en Rust : ${t1Rust - t0Rust} ms`);

    const t0Js = performance.now();
    const primesWithJs = primes_up_to(10000000);
    const t1Js = performance.now();
    console.log(`Dernier nombre premier jusqu'à 10000000 en Javascript : ${t1Js - t0Js} ms`);
});
