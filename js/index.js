import Vue from 'vue';
import mitt from 'mitt';

const emitter = mitt();

const app = new Vue({
    el: '#app',
    data: {
        running: false,
        results: {
            javascript: [],
            webassembly: []
        },
        stats: {
            javascript: {
                totalTime: 0,
                meanTime: 0
            },
            webassembly: {
                totalTime: 0,
                meanTime: 0
            }
        }
    },
    methods: {
        run: function() {
            this.running = true;

            setTimeout(() => emitter.emit('run'), 0);
        },
        stop: function(results = { javascript: [], webassembly: [] }) {
            this.running = false;
            this.results = results;
        }
    }
});

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
    console.log('Loaded WebAssembly module');
    emitter.on('run', () => {
        console.log('Running benchmark');
        const results = { webassembly: [], javascript: [] };

        for (let index = 1; index <= 2; index++) {
            const t0Rust = performance.now();
            const primesWithRust = module.primes_up_to(500000);
            const t1Rust = performance.now();

            const t0Js = performance.now();
            const primesWithJs = primes_up_to(500000);
            const t1Js = performance.now();

            results.webassembly.push({ round: index, time: t1Rust - t0Rust });
            results.javascript.push({ round: index, time: t1Js - t0Js });
        }

        app.stop(results);
    });
});
