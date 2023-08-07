// a(n, m) = (m + 1) * n + m * (m + 1) / 2
// ((m + 1) * n + m * (m + 1) / 2) / (n + m) = 

use primal::Sieve;

fn a(n: u128, m: u128) -> u128 {
    return (m + 1) * n + m * (m + 1) / 2;
}

fn U(n: u128, sieve: &Sieve) -> u128 {
    return (3..=n).map(|i| T(i, sieve)).sum();
}

fn T(n: u128, sieve: &Sieve) -> u128 {
    return S(n, sieve).iter().sum();
}

fn S(n: u128, sieve: &Sieve) -> Vec<u128> {
    let ndivs = divisors(n, sieve);
    let n1divs = divisors(n - 1, sieve);
    let mut res = Vec::new();
    for d1 in ndivs.iter() {
        for d2 in n1divs.iter() {
            let x = d1 * d2;
            if x <= n {
                continue;
            }
            let m = x - n;
            if a(n, m) % (n + m) == 0 {
                res.push(m);
            }
        }
    }
    return res;
}

fn divisors(n: u128, sieve: &Sieve) -> Vec<u128> {
    let factors: Vec<(usize, usize)> = sieve.factor(n as usize).unwrap();
    let mut res: Vec<u128> = vec![1];
    for f in 0..factors.len() {
        let div_cnt = res.len();
        for exp in 1..=factors[f].1 {
            for i in 0..div_cnt {
                res.push(res[i] * u128::try_from(factors[f].0.pow(exp as u32)).unwrap());
            }
        }
    }
    res.sort();
    return res;
}

fn main() {
    let n: u128 = 1234567;
    let sieve = Sieve::new(n.try_into().unwrap());
    println!("U({:?}) = {:?}", n, U(n, &sieve));
}