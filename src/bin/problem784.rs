use modinverse::modinverse;
use primal::Sieve;

fn main() {
    println!("{:?}", solve());
}

fn solve() -> i64 {
    // let n: i64 = 100;
    let n: i64 = 2000000;
    let sieve = Sieve::new(n as usize);
    let mut res = 0;
    for p in 2..=n {
        println!("{}", p);
        for qq in 2..p {
            let inv = modinverse(qq, p);
            match inv {
                None => continue,
                Some(r) => {
                    assert!(r * qq % p == 1);
                    // println!("{} {} {}", p, q, r);
                    let divs = divisors(p * r - 1, &sieve);
                    for div in divs {
                        if div < p {
                            continue;
                        }
                        if (div - qq) % p != 0 {
                            continue;
                        }
                        let q = div;
                        // println!("{} {} {}", p, q, r);
                        res += p + q;
                    }
                }
            }
        }
    }
    return res;
}

fn divisors(n: i64, sieve: &Sieve) -> Vec<i64> {
    let factors: Vec<(usize, usize)> = sieve.factor(n as usize).unwrap();
    let mut res: Vec<i64> = vec![1];
    for f in 0..factors.len() {
        let div_cnt = res.len();
        for exp in 1..=factors[f].1 {
            for i in 0..div_cnt {
                res.push(res[i] * i64::try_from(factors[f].0.pow(exp as u32)).unwrap());
            }
        }
    }
    res.sort();
    return res;
}

// qq < p
// p * r - 1 = 0 (mod k * p + qq)
// k * p + qq | p * r - 1