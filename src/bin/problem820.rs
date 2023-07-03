use std::collections::HashMap;
use std::thread;

fn d(n: i64, k: i64) -> i8 {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    let mut s: Vec<i8> = Vec::new();
    let mut rem: i64 = 1;
    let mut idx = 0;
    while !seen.contains_key(&rem) {
        seen.insert(rem, idx);
        let div = (rem / k) as i8;
        rem = (rem - div as i64 * k) * 10;
        s.push(div);
        idx += 1;
    }
    // println!("{:?}", s);
    let offset = *seen.get(&rem).unwrap();
    let period = idx - offset;
    // println!("offset={:?} period={:?}", offset, period);
    return if n <= offset { s[n as usize] } else { s[(offset + (n - offset) % period) as usize] };
}

fn s(n: i64, ks: Vec<i64>) -> i64 {
    let mut sum = 0;
    for k in ks.iter() {
        if k % 10000 == 0 {
            println!("{:?}", k);
        }
        sum += d(n, *k) as i64;
    }
    return sum;
}

fn calc(n: i64) -> i64 {
    let ks: Vec<i64> = (1..n + 1).collect();
    let workers = thread::available_parallelism().unwrap().get();
    let mut results: Vec<thread::JoinHandle<i64>> = Vec::new();
    for chunk in ks.chunks((ks.len() - 1) / workers + 1).map(|c| c.to_vec()) {
        results.push(thread::spawn(move || {
            return s(n, chunk.to_vec());
        }));
    }
    return results.into_iter().map(|r| r.join().unwrap()).sum();
}

fn main() {
    println!("{:?}", calc(10_i64.pow(7)));
}

// floor(10^n / k) = d + 10*e