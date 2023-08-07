use num::Zero;
use num_bigint::BigInt;

#[derive(Debug)]
struct NumberAndLog {
    i: u64,
    il: u64,
    l: f64,
}

fn main() {
    let n: u64 = 10_u64.pow(4);
    let m: u64 = 10_u64.pow(16);
    let maxl: u64 = n.ilog2().ilog2().into();
    let mut start: u64 = 0;
    let mut nums: Vec<NumberAndLog> = Vec::new();
    for i in 2..(n + 1) {
        let l = (i as f64).log2().log2();
        let il: u64 = i.ilog2().ilog2().into();
        start += maxl - il;
        nums.push(NumberAndLog {
            i,
            il,
            l: l - l.floor(),
        });
    }
    nums.sort_by(|a, b| a.l.partial_cmp(&b.l).unwrap());
    assert!(m >= start);
    let full_rounds = (m - start) / (n - 1);
    let rem = m - start - full_rounds * (n - 1);
    let finall = maxl + full_rounds;
    println!(
        "start={:?} full_rounds={:?} rem={:?} finall={:?}",
        start, full_rounds, rem, finall
    );

    let mut res: BigInt = BigInt::zero();
    let mm: u64 = 1234567891;
    for i in 0..(n - 1) {
        let exp2 =
            (if i < rem { finall + 1 } else { finall }) - nums[usize::try_from(i).unwrap()].il;
        let p2 = BigInt::from(2).modpow(&BigInt::from(exp2), &BigInt::from(mm - 1));
        let bi = BigInt::from(nums[usize::try_from(i).unwrap()].i).modpow(&p2, &BigInt::from(mm));
        res += bi;
    }
    res = res % 1234567891;

    println!("n={:?} m={:?} {:?}", n, m, res);
}
