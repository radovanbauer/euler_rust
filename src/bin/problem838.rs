use std::collections::{HashSet, HashMap};

use primal::Sieve;

fn main() {
    println!("{:.6?}", solve());
}

fn solve() -> f64 {
    let n = 1000000;
    let sieve = Sieve::new(1000000);
    let mut chosen_primes: HashSet<usize> = HashSet::new();
    let mut remaining: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 1..=n {
        if i % 10 != 3 {
            continue;
        }
        let res = sieve.factor(i).unwrap();
        let primes: Vec<usize> = res.iter().map(|x| x.0).collect();
        if primes.len() == 1 {
            chosen_primes.insert(primes[0]);
        } else {
            remaining.insert(i, primes);
        }
    }
    clean(&mut remaining, &chosen_primes);
    while !remaining.is_empty() {
        let remaining_primes: HashSet<usize> = remaining.values().flatten().cloned().collect();
        let mut remaining_primes_count: HashMap<usize, usize> = HashMap::new();
        for val in remaining.values() {
            for p in val {
                *remaining_primes_count.entry(*p).or_insert(0) += 1;
            }
        }
        let next = remaining_primes.iter().max_by_key(|p| (remaining_primes_count[p], **p)).unwrap().clone();
        chosen_primes.insert(next);
        clean(&mut remaining, &chosen_primes);
        println!("next={:?} remaining={:?}", next, remaining);
    }
    let res: f64 = chosen_primes.iter().map(|p| (*p as f64).ln()).sum();
    return res;
}

fn clean(remaining: &mut HashMap<usize, Vec<usize>>, chosen_primes: &HashSet<usize>) {
    let keys: Vec<usize> = remaining.keys().cloned().collect();
    for k in keys {
        if remaining[&k].iter().any(|p| chosen_primes.contains(p)) {
            remaining.remove(&k);
        }
    }
}