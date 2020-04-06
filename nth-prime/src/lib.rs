pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }
    let mut primes: Vec<u32> = vec![2, 3];
    let mut number = 3;
    let n_usize = n as usize;
    while primes.len() < n_usize + 1 {
        let mut is_prime = true;
        for i in 0..primes.len() {
            if number % primes[i] == 0 {
                is_prime = false;
            }
        }
        if is_prime && number != 3 {
            primes.push(number);
        }
        number += 2;
    }
    primes[n_usize]
}

