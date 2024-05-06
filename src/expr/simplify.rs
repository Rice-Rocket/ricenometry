use super::Expr;
use crate::prelude::*;

impl Expr {
    pub fn simplify(self) -> Self {
        match self {
            Expr::Integer(c) => Expr::Integer(c),
            Expr::Decimal(c) => Expr::Decimal(c),
            Expr::Variable(s) => Expr::Variable(s),
            Expr::Negation(v) => match v.clone().simplify() {
                Expr::Integer(x) => Expr::integer(-x),
                Expr::Decimal(x) => Expr::decimal(-x),
                other => Expr::negation(other.boxed()),
            },
            Expr::Sum { left, right } => match (left.clone().simplify(), right.clone().simplify()) {
                (Expr::Integer(a), Expr::Integer(b)) => Expr::integer(a + b),
                (Expr::Decimal(a), Expr::Decimal(b)) => Expr::decimal(a + b),
                (Expr::Integer(a), Expr::Decimal(b)) | (Expr::Decimal(b), Expr::Integer(a)) => Expr::decimal(a as f64 + b),

                (Expr::Integer(x), Expr::Ratio { numerator, denominator }) | (Expr::Ratio { numerator, denominator }, Expr::Integer(x)) =>
                    Expr::ratio(
                        Expr::sum(numerator, Expr::product(Expr::integer(x).boxed(), denominator.clone())
                            .simplify().boxed()).simplify().boxed(),
                        denominator,
                    ).simplify(),

                (Expr::Ratio { numerator: n1, denominator: d1 }, Expr::Ratio { numerator: n2, denominator: d2 }) => {
                    let d = Expr::product(d1.clone(), d2.clone()).simplify();
                    let n = Expr::sum(Expr::product(n1, d2).simplify().boxed(), Expr::product(n2, d1).simplify().boxed()).simplify();
                    Expr::ratio(n.boxed(), d.boxed()).simplify()
                },

                _ => Expr::sum(left, right)
            },
            Expr::Difference { left, right } => Expr::sum(left, Expr::negation(right).simplify().boxed()).simplify(),
            Expr::Product { left, right } => match (left.clone().simplify(), right.clone().simplify()) {
                (Expr::Integer(a), Expr::Integer(b)) => Expr::integer(a * b),
                (Expr::Decimal(a), Expr::Decimal(b)) => Expr::decimal(a * b),
                (Expr::Integer(a), Expr::Decimal(b)) | (Expr::Decimal(b), Expr::Integer(a)) => Expr::decimal(a as f64 * b),

                (Expr::Integer(x), Expr::Ratio { numerator, denominator }) | (Expr::Ratio { numerator, denominator }, Expr::Integer(x)) =>
                    Expr::ratio(Expr::product(Expr::integer(x).boxed(), denominator.clone()).simplify().boxed(), denominator).simplify(),

                (Expr::Ratio { numerator: n1, denominator: d1 }, Expr::Ratio { numerator: n2, denominator: d2 }) =>
                    Expr::ratio(Expr::product(n1, n2).simplify().boxed(), Expr::product(d1, d2).simplify().boxed()).simplify(),

                _ => Expr::product(left, right)
            },
            Expr::Ratio { numerator, denominator } => match (numerator.clone().simplify(), denominator.clone().simplify()) {
                (Expr::Integer(n), Expr::Integer(d)) => if d == 0 {
                    Expr::ratio(Expr::integer(n).boxed(), Expr::integer(d).boxed())
                } else if n == 0 {
                    Expr::integer(0)
                } else if n % d == 0 {
                    Expr::integer(n / d)
                } else {
                    let cd = signed_gcd(n, d);
                    Expr::ratio(Expr::integer(n / cd).boxed(), Expr::integer(d / cd).boxed())
                },

                _ => Expr::ratio(numerator, denominator),
            },
            Expr::Power { base, exp } => todo!(),
            Expr::Root { index, radicand } => todo!(),
            Expr::Equals { left, right } => todo!(),
            Expr::NotEquals { left, right } => todo!(),
            Expr::GreaterThan { left, right } => todo!(),
            Expr::LessThan { left, right } => todo!(),
            Expr::GreaterThanEq { left, right } => todo!(),
            Expr::LessThanEq { left, right } => todo!(),

        }
    }
}


fn signed_gcd(a: i64, b: i64) -> i64 {
    gcd(a.unsigned_abs(), b.unsigned_abs()) as i64
}


// https://www.wikiwand.com/en/Binary_GCD_algorithm
fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    let i = a.trailing_zeros(); a >>= i;
    let j = b.trailing_zeros(); b >>= j;
    let k = i.min(j);

    loop {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= a;

        if b == 0 {
            return a << k;
        }

        b >>= b.trailing_zeros();
    }
}
