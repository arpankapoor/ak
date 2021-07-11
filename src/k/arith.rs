use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

use crate::error::RuntimeErrorCode;
use crate::k::{KResult, K, K0};

macro_rules! impl_i64_arith {
    ($trait: tt, $method: tt, $op: tt) => {
        impl $trait<i64> for &K {
            type Output = KResult;

            fn $method(self, rhs: i64) -> Self::Output {
                match self.deref() {
                    K0::Int(x) => Ok(K0::Int(x $op rhs).into()),
                    K0::Float(x) => Ok(K0::Float(x $op rhs as f64).into()),
                    K0::IntList(x) => Ok(K0::IntList(x.iter().map(|i| i $op rhs).collect()).into()),
                    K0::FloatList(x) => {
                        let rhs = rhs as f64;
                        Ok(K0::FloatList(x.iter().map(|i| i $op rhs).collect()).into())
                    }
                    K0::GenList(x) => Ok(x
                        .iter()
                        .map(|i| i $op rhs)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),
                    _ => Err(RuntimeErrorCode::Type),
                }
            }
        }

        impl $trait<&K> for i64 {
            type Output = KResult;

            fn $method(self, rhs: &K) -> Self::Output {
                match rhs.deref() {
                    K0::Int(x) => Ok(K0::Int(self $op x).into()),
                    K0::Float(x) => Ok(K0::Float(self as f64 $op x).into()),
                    K0::IntList(x) => Ok(K0::IntList(x.iter().map(|i| self $op i).collect()).into()),
                    K0::FloatList(x) => {
                        let lhs = self as f64;
                        Ok(K0::FloatList(x.iter().map(|i| lhs $op i).collect()).into())
                    }
                    K0::GenList(x) => Ok(x
                        .iter()
                        .map(|i| self $op i)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),
                    _ => Err(RuntimeErrorCode::Type),
                }
            }
        }
    };
}

macro_rules! impl_f64_arith {
    ($trait: tt, $method: tt, $op: tt) => {
        impl $trait<f64> for &K {
            type Output = KResult;

            fn $method(self, rhs: f64) -> Self::Output {
                match self.deref() {
                    K0::Int(x) => Ok(K0::Float(*x as f64 $op rhs).into()),
                    K0::Float(x) => Ok(K0::Float(x $op rhs).into()),
                    K0::IntList(x) => {
                        Ok(K0::FloatList(x.iter().map(|&i| i as f64 $op rhs).collect()).into())
                    }
                    K0::FloatList(x) => {
                        Ok(K0::FloatList(x.iter().map(|i| i $op rhs).collect()).into())
                    }
                    K0::GenList(x) => Ok(x
                        .iter()
                        .map(|i| i $op rhs)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),
                    _ => Err(RuntimeErrorCode::Type),
                }
            }
        }

        impl $trait<&K> for f64 {
            type Output = KResult;

            fn $method(self, rhs: &K) -> Self::Output {
                match rhs.deref() {
                    K0::Int(x) => Ok(K0::Float(self $op *x as f64).into()),
                    K0::Float(x) => Ok(K0::Float(self $op x).into()),
                    K0::IntList(x) => {
                        Ok(K0::FloatList(x.iter().map(|&i| self $op i as f64).collect()).into())
                    }
                    K0::FloatList(x) => {
                        Ok(K0::FloatList(x.iter().map(|i| self $op i).collect()).into())
                    }
                    K0::GenList(x) => Ok(x
                        .iter()
                        .map(|i| self $op i)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),
                    _ => Err(RuntimeErrorCode::Type),
                }
            }
        }
    };
}

macro_rules! impl_k_arith {
    ($trait: tt, $method: tt, $op: tt) => {
        impl $trait for &K {
            type Output = KResult;

            fn $method(self, rhs: Self) -> Self::Output {
                match (self.deref(), rhs.deref()) {
                    (K0::Int(x), _) => *x $op rhs,
                    (K0::Float(x), _) => *x $op rhs,

                    (K0::IntList(x), K0::IntList(y)) => {
                        if x.len() == y.len() {
                            Ok(K0::IntList(x.iter().zip(y).map(|(i, j)| i $op j).collect()).into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::IntList(x), K0::FloatList(y)) => {
                        if x.len() == y.len() {
                            Ok(
                                K0::FloatList(
                                    x.iter().zip(y).map(|(&i, j)| i as f64 $op j).collect(),
                                )
                                .into(),
                            )
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::IntList(x), K0::GenList(y)) => {
                        if x.len() == y.len() {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(&i, j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::IntList(x), _) => Ok(x
                        .iter()
                        .map(|&i| i $op rhs)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),

                    (K0::FloatList(x), K0::IntList(y)) => {
                        if x.len() == y.len() {
                            Ok(
                                K0::FloatList(
                                    x.iter().zip(y).map(|(i, &j)| i $op j as f64).collect(),
                                )
                                .into(),
                            )
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::FloatList(x), K0::FloatList(y)) => {
                        if x.len() == y.len() {
                            Ok(K0::FloatList(x.iter().zip(y).map(|(i, j)| i $op j).collect()).into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::FloatList(x), K0::GenList(y)) => {
                        if x.len() == y.len() {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(&i, j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::FloatList(x), _) => Ok(x
                        .iter()
                        .map(|&i| i $op rhs)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),

                    (K0::GenList(x), K0::IntList(y)) => {
                        if x.len() == y.len() {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(i, &j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::GenList(x), K0::FloatList(y)) => {
                        if x.len() == y.len() {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(i, &j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::GenList(x), K0::GenList(y)) => {
                        if x.len() == y.len() {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(i, j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        } else {
                            Err(RuntimeErrorCode::Length)
                        }
                    }
                    (K0::GenList(x), _) => Ok(x
                        .iter()
                        .map(|i| i $op rhs)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),

                    (_, _) => Err(RuntimeErrorCode::Type),
                }
            }
        }
    };
}

impl_i64_arith!(Add, add, +);
impl_f64_arith!(Add, add, +);
impl_k_arith!(Add, add, +);

impl_i64_arith!(Sub, sub, -);
impl_f64_arith!(Sub, sub, -);
impl_k_arith!(Sub, sub, -);

impl_i64_arith!(Mul, mul, *);
impl_f64_arith!(Mul, mul, *);
impl_k_arith!(Mul, mul, *);

impl_f64_arith!(Div, div, /);

// convert ints to floats for division
impl Div<i64> for &K {
    type Output = KResult;

    fn div(self, rhs: i64) -> Self::Output {
        self / rhs as f64
    }
}

impl Div<&K> for i64 {
    type Output = KResult;

    fn div(self, rhs: &K) -> Self::Output {
        self as f64 / rhs
    }
}

impl Div for &K {
    type Output = KResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.deref(), rhs.deref()) {
            (K0::Int(x), _) => *x / rhs,
            (K0::Float(x), _) => *x / rhs,

            (K0::IntList(x), K0::IntList(y)) => {
                if x.len() == y.len() {
                    Ok(K0::FloatList(
                        x.iter()
                            .zip(y)
                            .map(|(&i, &j)| i as f64 / j as f64)
                            .collect(),
                    )
                    .into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::IntList(x), K0::FloatList(y)) => {
                if x.len() == y.len() {
                    Ok(K0::FloatList(x.iter().zip(y).map(|(&i, j)| i as f64 / j).collect()).into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::IntList(x), K0::GenList(y)) => {
                if x.len() == y.len() {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(&i, j)| i as f64 / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::IntList(x), _) => Ok(x
                .iter()
                .map(|&i| i / rhs)
                .collect::<Result<Vec<_>, _>>()?
                .into()),

            (K0::FloatList(x), K0::IntList(y)) => {
                if x.len() == y.len() {
                    Ok(K0::FloatList(x.iter().zip(y).map(|(i, &j)| i / j as f64).collect()).into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::FloatList(x), K0::FloatList(y)) => {
                if x.len() == y.len() {
                    Ok(K0::FloatList(x.iter().zip(y).map(|(i, j)| i / j).collect()).into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::FloatList(x), K0::GenList(y)) => {
                if x.len() == y.len() {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(&i, j)| i / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::FloatList(x), _) => Ok(x
                .iter()
                .map(|&i| i / rhs)
                .collect::<Result<Vec<_>, _>>()?
                .into()),

            (K0::GenList(x), K0::IntList(y)) => {
                if x.len() == y.len() {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(i, &j)| i / j as f64)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::GenList(x), K0::FloatList(y)) => {
                if x.len() == y.len() {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(i, &j)| i / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::GenList(x), K0::GenList(y)) => {
                if x.len() == y.len() {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(i, j)| i / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                } else {
                    Err(RuntimeErrorCode::Length)
                }
            }
            (K0::GenList(x), _) => Ok(x
                .iter()
                .map(|i| i / rhs)
                .collect::<Result<Vec<_>, _>>()?
                .into()),

            (_, _) => Err(RuntimeErrorCode::Type),
        }
    }
}

impl Neg for &K {
    type Output = KResult;

    fn neg(self) -> Self::Output {
        match self.deref() {
            K0::Int(x) => Ok(K0::Int(-x).into()),
            K0::Float(x) => Ok(K0::Float(-x).into()),
            K0::IntList(x) => Ok(K0::IntList(x.iter().map(|i| -i).collect()).into()),
            K0::FloatList(x) => Ok(K0::FloatList(x.iter().map(|i| -i).collect()).into()),
            K0::GenList(x) => Ok(x.iter().map(|i| -i).collect::<Result<Vec<_>, _>>()?.into()),
            _ => Err(RuntimeErrorCode::Type),
        }
    }
}
