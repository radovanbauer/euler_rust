// p(0) = 2, p(1) = 12, n >= 2: p(n) = 6*p(n-1) - p(n-2)
// s(n) = p(1) + sum(6*p(i - 1) - p(i - 2), i, 2, n) = p(1) + 6*s(n - 1) - p(0) - s(n - 2) = 10 + 6*s(n - 1) - s(n - 2)

use std::ops::Add;

use nalgebra::{
    allocator::Allocator,
    constraint::{DimEq, ShapeConstraint},
    matrix, vector, ClosedAdd, ClosedMul, ClosedSub, DefaultAllocator, Dim, Matrix, Matrix2,
    Matrix2x1, Matrix3, RawStorage, Scalar, Storage, Vector2, Vector3, Matrix4, Vector4,
};
use num_traits::identities::{One, Zero};
use rug::{ops::Pow, Float, Integer};

fn main() {
    println!("{:?}", solve());
}

#[derive(Debug, Clone, PartialEq)]
struct ModNum<const M: u64> {
    num: u64,
}

impl<const M: u64> ModNum<M> {
    fn new(num: u64) -> Self {
        ModNum { num: num % M }
    }
}

impl<const M: u64> Zero for ModNum<M> {
    fn zero() -> Self {
        ModNum::new(0)
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }
}

impl<const M: u64> One for ModNum<M> {
    fn one() -> Self {
        ModNum::new(1)
    }
}

impl<const M: u64> std::ops::Add for ModNum<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ModNum::new((self.num + rhs.num) % M)
    }
}

impl<const M: u64> std::ops::AddAssign for ModNum<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.num = (self.num + rhs.num) % M;
    }
}

impl<const M: u64> std::ops::Mul for ModNum<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ModNum::new((self.num * rhs.num) % M)
    }
}

impl<const M: u64> std::ops::MulAssign for ModNum<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.num = (self.num * rhs.num) % M;
    }
}

impl<const M: u64> std::ops::Sub for ModNum<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ModNum::new((self.num + M - rhs.num) % M)
    }
}

impl<const M: u64> std::ops::SubAssign for ModNum<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.num = (self.num - rhs.num) % M;
    }
}


#[derive(Debug, Clone, PartialEq)]
struct Int {
    num: Integer,
}

impl Zero for Int {
    fn zero() -> Self {
        Int::new(Integer::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.num == Integer::ZERO
    }
}

impl One for Int {
    fn one() -> Self {
        Int::from_u64(1)
    }
}

impl std::ops::Add for Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Int::new(self.num + rhs.num)
    }
}

impl std::ops::AddAssign for Int {
    fn add_assign(&mut self, rhs: Self) {
        self.num = &self.num + rhs.num;
    }
}

impl std::ops::Mul for Int {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Int::new(self.num * rhs.num)
    }
}

impl std::ops::MulAssign for Int {
    fn mul_assign(&mut self, rhs: Self) {
        self.num = &self.num * rhs.num;
    }
}

impl std::ops::Sub for Int {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Int::new(self.num - rhs.num)
    }
}

impl std::ops::SubAssign for Int {
    fn sub_assign(&mut self, rhs: Self) {
        self.num = &self.num - rhs.num;
    }
}

impl Int {
    fn from_u64(num: u64) -> Self {
        Int { num: Integer::from(num) }
    }

    fn from_i64(num: i64) -> Self {
        Int { num: Integer::from(num) }
    }

    fn new(num: Integer) -> Self {
        Int { num: num }
    }
}

fn pow<T>(mat: Matrix4<T>, mut exp: Integer) -> Matrix4<T>
where T: Scalar + Zero + One + ClosedAdd + ClosedMul + ClosedSub {
    let mut base = mat;
    let mut acc = base.clone();
    acc.set_one();

    while exp > Integer::from(1_u64) {
        if exp.is_odd() {
            acc = acc * &base;
        }
        exp /= 2;
        base = &base * &base;
    }

    // since exp!=0, finally the exp must be 1.
    // Deal with the final bit of the exponent separately, since
    // squaring the base afterwards is not necessary and may cause a
    // needless overflow.
    acc * base
}

const MOD: u64 = 1234567891_u64;

fn solve() -> u64 {
    let n: Integer = Integer::from(10_u64).pow(100000).pow(100000);
    // let n: Integer = Integer::from(10000_u64);
    const PREC: u32 = 200;
    let nlog10 = Float::with_val(PREC, 10000000000_u64);
    // let nlog10 = Float::with_val(PREC, 4_u64);
    let m: Integer = Integer::from(1234567891_u64);

    let mut res: Integer = Integer::from(0_u64);

    // let exp: Integer = ((nlog10
    //     - (Float::with_val(PREC, 10)
    //         + Float::with_val(PREC, 7) * Float::with_val(PREC, 2).sqrt())
    //     .log10())
    //     / (Float::with_val(PREC, 3) + Float::with_val(PREC, 2) * Float::with_val(PREC, 2)).log10())
    // .round()
    // .to_integer()
    // .unwrap();
    // println!("exp: {}", &exp);

    let per_mat: Matrix4<Int> = matrix![
        Int::from_u64(6), Int::from_i64(-1), Int::from_u64(0), Int::from_u64(0);
        Int::from_u64(1), Int::from_u64(0), Int::from_u64(0), Int::from_u64(0);
        Int::from_u64(0), Int::from_u64(0), Int::from_u64(1), Int::from_u64(0);
        Int::from_u64(1), Int::from_u64(0), Int::from_u64(0), Int::from_u64(1)];
    let per_vec: Vector4<Int> = vector![Int::from_u64(12), Int::from_u64(2), Int::from_u64(1), Int::from_u64(0)];

    let mut exp_lo: Integer = Integer::from(0_u64);
    let mut exp_hi: Integer = nlog10.to_integer().unwrap() * 2;

    while exp_hi.clone() - exp_lo.clone() > 1 {
        println!("{} {}", exp_lo, exp_hi);
        let exp_mid: Integer = (exp_lo.clone() + exp_hi.clone()) / 2;
        let per_mat_pow = pow(per_mat.clone(), exp_mid.clone());
        let per_vec_pow = per_mat_pow * per_vec.clone();
        if per_vec_pow.y.num > n {
            exp_hi = exp_mid;
        } else {
            exp_lo = exp_mid;
        }
    }
    println!("{} {}", exp_lo, exp_hi);
    let per_vec_pow = pow(per_mat.clone(), exp_lo.clone()) * per_vec.clone();
    println!("per_vec_pow: {:?}", per_vec_pow);
    res = (res + (per_vec_pow.w.num.clone() % &m).to_u64().unwrap()) % &m;

    // println!("per_mat_pow: {:?}", per_mat_pow);

    // let mat: Matrix3<ModNum<MOD>> = matrix![
    //     ModNum::new(6), ModNum::new(MOD - 1), ModNum::new(10);
    //     ModNum::new(1), ModNum::new(0), ModNum::new(0);
    //     ModNum::new(0), ModNum::new(0), ModNum::new(1)];
    // println!("mat: {:?}", mat);

    // let vec: Vector3<ModNum<MOD>> = vector![ModNum::new(12), ModNum::new(0), ModNum::new(1)];

    // let mat_pow = pow(mat, exp.clone()) * vec;
    // println!("mat_pow: {:?}", mat_pow);
    // res = (res + mat_pow.x.num) % &m;

    // let mut per: Integer = Integer::from(12_u64);
    // let mut prev_per = Integer::from(2_u64);
    // loop {
    //     if per > n {
    //         break;
    //     }

    //     res = (res + &per) % &m;
    //     println!("{} {}", per, res);

    //     let next_per = (&per) * Integer::from(6_u64) - prev_per;
    //     prev_per = per;
    //     per = next_per;
    // }

    let mut max_a_sq = ((&n) * Integer::from(2_u64)
        - ((&n) * Integer::from(4_u64) + Integer::from(1_u64)).sqrt()
        + Integer::from(1_u64))
        / Integer::from(2_u64);
    if max_a_sq.clone() + max_a_sq.clone().sqrt() > n {
        max_a_sq -= Integer::from(2_u64);
    }
    // println!("max_a_sq: {}", max_a_sq);
    let max_a = max_a_sq.sqrt();
    let max_b = (max_a.clone().pow(2_u32) - Integer::from(1_u64)) / Integer::from(2_u64);
    let max_c = max_b.clone() + Integer::from(1_u64);
    // println!("{} {} {}", max_a, max_b, max_c);
    assert!(max_a.clone().pow(2_u32) + max_b.clone().pow(2_u32) == max_c.clone().pow(2_u32));
    assert!(max_a.clone() + max_b.clone() + max_c.clone() <= n);

    let next_a = max_a.clone() + Integer::from(2_u64);
    let next_b = (next_a.clone().pow(2_u32) - Integer::from(1_u64)) / Integer::from(2_u64);
    let next_c = next_b.clone() + Integer::from(1_u64);
    assert!(next_a.clone().pow(2_u32) + next_b.clone().pow(2_u32) == next_c.clone().pow(2_u32));
    assert!(next_a.clone() + next_b.clone() + next_c.clone() > n);

    let min_a = Integer::from(5_u64);
    res = (res
        + ((&max_a) * Integer::from(10_u64)
            + max_a.clone().pow(2) * Integer::from(9_u64)
            + max_a.clone().pow(3) * Integer::from(2_u64)
            + min_a.clone() * Integer::from(2_u64)
            + min_a.clone().pow(2) * Integer::from(3_u64)
            - min_a.pow(3) * Integer::from(2_u64))
            / Integer::from(12_u64))
        % &m;

    // a = min_a;
    // while a <= max_a {
    //     let b = ((&a).pow(2_u32) - Integer::from(1_u64)) / Integer::from(2_u64);
    //     let c = (&b) + Integer::from(1_u64);
    //     assert!((&a).pow(2_u32) + (&b).pow(2_u32) == (&c).pow(2_u32));
    //     res = (res + &a + &b + &c) % &m;
    //     println!("{} {} {} {}", a, b, c, res);
    //     a += Integer::from(2_u64);
    // }

    // sum((a*2 - 1) / 2, min_a, max_a) = sum()

    return res.try_into().unwrap();
}
