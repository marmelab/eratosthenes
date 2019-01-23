const primes_up_to = max => {
    var sieve = [], i, j, primes = [];
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
    console.log('test Rust', module.primes_up_to(300));
    console.log('test Javascript', primes_up_to(300));
});
