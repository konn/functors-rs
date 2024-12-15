use std::ops::{Add, Mul};

pub mod functor;

pub trait Language: Sized {
    type Layer<T>;

    fn fmap<T, S, F>(f: F, layer: Self::Layer<T>) -> Self::Layer<S>
    where
        F: FnMut(T) -> S;
    fn wrap(layer: Self::Layer<Self>) -> Self;
    fn unwrap(self) -> Self::Layer<Self>;

    fn fold<T, F>(self, f: &mut F) -> T
    where
        F: FnMut(Self::Layer<T>) -> T,
    {
        let ft = Self::fmap(|me| me.fold(f), self.unwrap());
        f(ft)
    }

    fn unfold<T, F>(f: &mut F, a: T) -> Self
    where
        F: FnMut(T) -> Self::Layer<T>,
    {
        let seed = f(a);
        Self::wrap(Self::fmap(|a| Self::unfold(f, a), seed))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Int(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExprF<T> {
    Int(i64),
    Var(String),
    Add(T, T),
    Mul(T, T),
}

impl Add for Expr {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Expr::Add(self.into(), other.into())
    }
}

impl Mul for Expr {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Expr::Mul(self.into(), other.into())
    }
}

impl From<&str> for Expr {
    fn from(s: &str) -> Self {
        Self::Var(s.to_string())
    }
}

impl From<String> for Expr {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl From<i64> for Expr {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl Language for Expr {
    type Layer<T> = ExprF<T>;

    fn fmap<T, S, F>(mut f: F, layer: Self::Layer<T>) -> Self::Layer<S>
    where
        F: FnMut(T) -> S,
    {
        match layer {
            ExprF::Int(n) => ExprF::Int(n),
            ExprF::Var(s) => ExprF::Var(s),
            ExprF::Add(a, b) => ExprF::Add(f(a), f(b)),
            ExprF::Mul(a, b) => ExprF::Mul(f(a), f(b)),
        }
    }

    fn wrap(layer: Self::Layer<Self>) -> Self {
        match layer {
            ExprF::Int(n) => Expr::Int(n),
            ExprF::Var(s) => Expr::Var(s),
            ExprF::Add(a, b) => a + b,
            ExprF::Mul(a, b) => a * b,
        }
    }

    fn unwrap(self) -> Self::Layer<Self> {
        match self {
            Expr::Int(n) => ExprF::Int(n),
            Expr::Var(s) => ExprF::Var(s),
            Expr::Add(a, b) => ExprF::Add(*a, *b),
            Expr::Mul(a, b) => ExprF::Mul(*a, *b),
        }
    }
}

impl Expr {
    pub fn eval(self) -> Result<i64, Self> {
        self.fold(&mut |layer| match layer {
            ExprF::Int(n) => Ok(n),
            ExprF::Var(s) => Err(Expr::Var(s)),
            ExprF::Add(Ok(a), Ok(b)) => Ok(a + b),
            ExprF::Add(a, b) => {
                Err(a.map_or_else(|e| e, Expr::from) + b.map_or_else(|e| e, Expr::from))
            }
            ExprF::Mul(Ok(a), Ok(b)) => Ok(a * b),
            ExprF::Mul(a, b) => {
                Err(a.map_or_else(|e| e, Expr::from) * b.map_or_else(|e| e, Expr::from))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(Expr::from(1) + 2.into(), 3; "1 + 2 = 3")]
    fn test_closed(expr: Expr, result: i64) {
        assert_eq!(expr.eval(), Ok(result));
    }
    #[test_case(Expr::from(1) + 2.into(), Ok(3); "1 + 2 = 3")]
    #[test_case((Expr::from(1) + 2.into()) * "x".into(), Err(Expr::from(3) * "x".into()); "(1 + 2) x = 3x")]
    fn test_any(expr: Expr, result: Result<i64, Expr>) {
        assert_eq!(expr.eval(), result);
    }
}
