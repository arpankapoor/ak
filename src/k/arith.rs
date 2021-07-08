use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::error::RuntimeErrorCode;
use crate::k::{KResult, K};

macro_rules! impl_i64_arith {
    ($trait: tt, $method: tt, $op: tt) => {
        impl $trait<i64> for &K {
            type Output = KResult;

            fn $method(self, rhs: i64) -> Self::Output {
                match self {
                    K::Int(x) => Ok(K::Int(x $op rhs)),
                    K::Float(x) => Ok(K::Float(x $op rhs as f64)),
                    K::IntList(x) => Ok(K::IntList(x.iter().map(|i| i $op rhs).collect())),
                    K::FloatList(x) => {
                        let rhs = rhs as f64;
                        Ok(K::FloatList(x.iter().map(|i| i $op rhs).collect()))
                    }
                    K::GenList(x) => Ok(x
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
                match rhs {
                    K::Int(x) => Ok(K::Int(self $op x)),
                    K::Float(x) => Ok(K::Float(self as f64 $op x)),
                    K::IntList(x) => Ok(K::IntList(x.iter().map(|i| self $op i).collect())),
                    K::FloatList(x) => {
                        let lhs = self as f64;
                        Ok(K::FloatList(x.iter().map(|i| lhs $op i).collect()))
                    }
                    K::GenList(x) => Ok(x
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
                match self {
                    K::Int(x) => Ok(K::Float(*x as f64 $op rhs)),
                    K::Float(x) => Ok(K::Float(x $op rhs)),
                    K::IntList(x) => Ok(K::FloatList(x.iter().map(|&i| i as f64 $op rhs).collect())),
                    K::FloatList(x) => Ok(K::FloatList(x.iter().map(|i| i $op rhs).collect())),
                    K::GenList(x) => Ok(x
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
                match rhs {
                    K::Int(x) => Ok(K::Float(self $op *x as f64)),
                    K::Float(x) => Ok(K::Float(self $op x)),
                    K::IntList(x) => Ok(K::FloatList(x.iter().map(|&i| self $op i as f64).collect())),
                    K::FloatList(x) => Ok(K::FloatList(x.iter().map(|i| self $op i).collect())),
                    K::GenList(x) => Ok(x
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
                match (self, rhs) {
                    (K::Int(x), y) => *x $op y,
                    (K::Float(x), y) => *x $op y,

                    (K::IntList(x), K::IntList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(K::IntList(x.iter().zip(y).map(|(i, j)| i $op j).collect()))
                        }
                    }
                    (K::IntList(x), K::FloatList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(K::FloatList(
                                x.iter().zip(y).map(|(&i, j)| i as f64 $op j).collect(),
                            ))
                        }
                    }
                    (K::IntList(x), K::GenList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(&i, j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        }
                    }
                    (K::IntList(x), y) => Ok(x
                        .iter()
                        .map(|&i| i $op y)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),

                    (K::FloatList(x), K::IntList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(K::FloatList(
                                x.iter().zip(y).map(|(i, &j)| i $op j as f64).collect(),
                            ))
                        }
                    }
                    (K::FloatList(x), K::FloatList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(K::FloatList(x.iter().zip(y).map(|(i, j)| i $op j).collect()))
                        }
                    }
                    (K::FloatList(x), K::GenList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(&i, j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        }
                    }
                    (K::FloatList(x), y) => Ok(x
                        .iter()
                        .map(|&i| i $op y)
                        .collect::<Result<Vec<_>, _>>()?
                        .into()),

                    (K::GenList(x), K::IntList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(i, &j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        }
                    }
                    (K::GenList(x), K::FloatList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(i, &j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        }
                    }
                    (K::GenList(x), K::GenList(y)) => {
                        if x.len() != y.len() {
                            Err(RuntimeErrorCode::Length)
                        } else {
                            Ok(x.iter()
                                .zip(y)
                                .map(|(i, j)| i $op j)
                                .collect::<Result<Vec<_>, _>>()?
                                .into())
                        }
                    }
                    (K::GenList(x), y) => Ok(x
                        .iter()
                        .map(|i| i $op y)
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
        match (self, rhs) {
            (K::Int(x), y) => *x / y,
            (K::Float(x), y) => *x / y,

            (K::IntList(x), K::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter()
                            .zip(y)
                            .map(|(&i, &j)| i as f64 / j as f64)
                            .collect(),
                    ))
                }
            }
            (K::IntList(x), K::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&i, j)| i as f64 / j).collect(),
                    ))
                }
            }
            (K::IntList(x), K::GenList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(&i, j)| i as f64 / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                }
            }
            (K::IntList(x), y) => Ok(x
                .iter()
                .map(|&i| i / y)
                .collect::<Result<Vec<_>, _>>()?
                .into()),

            (K::FloatList(x), K::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(i, &j)| i / j as f64).collect(),
                    ))
                }
            }
            (K::FloatList(x), K::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(x.iter().zip(y).map(|(i, j)| i / j).collect()))
                }
            }
            (K::FloatList(x), K::GenList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(&i, j)| i / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                }
            }
            (K::FloatList(x), y) => Ok(x
                .iter()
                .map(|&i| i / y)
                .collect::<Result<Vec<_>, _>>()?
                .into()),

            (K::GenList(x), K::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(i, &j)| i / j as f64)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                }
            }
            (K::GenList(x), K::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(i, &j)| i / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                }
            }
            (K::GenList(x), K::GenList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(x.iter()
                        .zip(y)
                        .map(|(i, j)| i / j)
                        .collect::<Result<Vec<_>, _>>()?
                        .into())
                }
            }
            (K::GenList(x), y) => Ok(x
                .iter()
                .map(|i| i / y)
                .collect::<Result<Vec<_>, _>>()?
                .into()),

            (_, _) => Err(RuntimeErrorCode::Type),
        }
    }
}

impl Neg for &K {
    type Output = KResult;

    fn neg(self) -> Self::Output {
        match self {
            K::Int(x) => Ok(K::Int(-x)),
            K::Float(x) => Ok(K::Float(-x)),
            K::IntList(x) => Ok(K::IntList(x.iter().map(|i| -i).collect())),
            K::FloatList(x) => Ok(K::FloatList(x.iter().map(|i| -i).collect())),
            K::GenList(x) => Ok(x.iter().map(|i| -i).collect::<Result<Vec<_>, _>>()?.into()),
            _ => Err(RuntimeErrorCode::Type),
        }
    }
}
