// f(n, k) = 1/2^k * (floor(n/2) + floor(n/4) + floor(n/8) + ...) + 1/3^k * (floor(n/3) + floor(n/9) + floor(n/81) + ...) + ...

use std::collections::{HashMap, HashSet};

use num::integer::Roots;
use primal::Sieve;

fn main() {
    let mut n = 1000;
    while n < 10_u64.pow(18) {
        println!("{} {:.12}", n, calc_f2(n, 1));
        n *= 10;
    }
}

fn calc_f1(n: u64, k: u64) -> f64 {
    println!("sieving");
    let nsqrt: u64 = n.sqrt();
    let sieve = Sieve::new(nsqrt.try_into().unwrap());
    let primes: Vec<u64> = sieve
        .primes_from(0)
        .map(|x| x.try_into().unwrap())
        .collect();

    println!("calculating");
    let mut res = 0_f64;
    let mut keys: Vec<u64> = Vec::new();
    for i in 0..=(n / (nsqrt + 1)) {
        keys.push(i);
    }
    for i in (1..=nsqrt).rev() {
        keys.push(n / i);
    }
    let mut key_to_idx = HashMap::new();
    for idx in 0..keys.len() {
        key_to_idx.insert(keys[idx], idx);
    }
    // println!("{:?}", keys);
    let mut cnt: Vec<u64> = Vec::new();
    for key in keys.iter().cloned() {
        cnt.push(0);
    }

    for pi in 0..primes.len() {
        let p = primes[pi];
        let mut prod = p * p;
        let mut new_cnt: Vec<u64> = Vec::new();
        println!("{} {} {} {}", n, nsqrt, p, cnt.len());
        for key in keys.iter().cloned() {
            if key > n / prod {
                break;
            }
            new_cnt.push(
                cnt.get(*key_to_idx.get(&key).unwrap()).unwrap() + key / p
                    - cnt.get(*key_to_idx.get(&(key / p)).unwrap()).unwrap(),
            );
        }
        while prod <= n {
            let x = n / prod - cnt.get(*key_to_idx.get(&(n / prod)).unwrap()).unwrap();
            res += (1_f64 / (p as f64).powf(k as f64)) * (x as f64);
            // println!("{} {} {} {}", p, prod, x, res);
            prod *= p;
        }
        cnt = new_cnt;
    }
    return res / (n as f64);
}

fn calc_f2(n: u64, k: u64) -> f64 {
    println!("sieving");
    let nsqrt: u64 = n.sqrt();
    let sieve = Sieve::new(nsqrt.try_into().unwrap());
    let primes: Vec<u64> = sieve
        .primes_from(0)
        .map(|x| x.try_into().unwrap())
        .collect();

    println!("calculating");
    let mut res = 0_f64;

    for pi in 0..primes.len() {
        let p = primes[pi];
        let mut prod = p;
        // println!("{} {} {}", n, nsqrt, p);
        while prod <= n / p {
            prod *= p;
            let x = calc_sum(n, &primes, prod, pi, 1);
            res += (1_f64 / ((p - 1) as f64)) * (x as f64);
            // println!("{} {} {} {}", p, prod, x, res);
        }
    }
    return res / (n as f64);
}

fn calc_sum(n: u64, primes: &Vec<u64>, prod: u64, max_pi: usize, sign: i128) -> i128 {
    let mut res = 0;
    res += sign * i128::try_from(n / prod).unwrap();
    for pi in 0..max_pi {
        let new_prod = prod * primes[pi];
        if new_prod > n {
            break;
        }
        res += calc_sum(n, primes, prod * primes[pi], pi, -sign);
    }
    return res;
}

// 1/p / (1 - 1/p) = 1 / (p - 1)