import('../crate/pkg').then(module => {
    console.log(module.add_one(41));
    console.log('test', module.primes_up_to(300));
});
