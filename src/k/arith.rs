use std::ops::{Add, Div, Mul, Sub};

use crate::error::RuntimeErrorCode;
use crate::k::K;

type KResult = Result<K, RuntimeErrorCode>;

macro_rules! arithmetic_operation {
    ($self: expr, $rhs: expr, $op: tt) => {
        match ($self, $rhs) {
            (K::Int(x), K::Int(y)) => Ok(K::Int(x $op y)),
            (K::Int(x), K::Float(y)) => Ok(K::Float(x as f64 $op y)),
            (K::Int(x), K::IntList(y)) => Ok(K::IntList(y.iter().map(|e| x $op e).collect())),
            (K::Int(x), K::FloatList(y)) => {
                let x = x as f64;
                Ok(K::FloatList(y.iter().map(|e| x $op e).collect()))
            }

            (K::Float(x), K::Int(y)) => Ok(K::Float(x $op y as f64)),
            (K::Float(x), K::Float(y)) => Ok(K::Float(x $op y)),
            (K::Float(x), K::IntList(y)) => {
                Ok(K::FloatList(y.iter().map(|&e| x $op e as f64).collect()))
            }
            (K::Float(x), K::FloatList(y)) => {
                Ok(K::FloatList(y.iter().map(|e| x $op e).collect()))
            }

            (K::IntList(x), K::Int(y)) => Ok(K::IntList(x.iter().map(|e| e $op y).collect())),
            (K::IntList(x), K::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|&e| e as f64 $op y).collect()))
            }
            (K::IntList(x), K::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::IntList(x.iter().zip(y).map(|(x, y)| x $op y).collect()))
                }
            }
            (K::IntList(x), K::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&x, y)| x as f64 $op y).collect(),
                    ))
                }
            }

            (K::FloatList(x), K::Int(y)) => {
                let y = y as f64;
                Ok(K::FloatList(x.iter().map(|e| e $op y).collect()))
            }
            (K::FloatList(x), K::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|e| e $op y).collect()))
            }
            (K::FloatList(x), K::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(x, y)| x $op y as f64).collect(),
                    ))
                }
            }
            (K::FloatList(x), K::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(x.iter().zip(y).map(|(&x, y)| x $op y).collect()))
                }
            }

            (_, _) => Err(RuntimeErrorCode::Type),
        }
    }
}

impl Add for K {
    type Output = KResult;

    fn add(self, rhs: Self) -> Self::Output {
        arithmetic_operation!(self, rhs, +)
    }
}

impl Sub for K {
    type Output = KResult;

    fn sub(self, rhs: Self) -> Self::Output {
        arithmetic_operation!(self, rhs, -)
    }
}

impl Mul for K {
    type Output = KResult;

    fn mul(self, rhs: Self) -> Self::Output {
        arithmetic_operation!(self, rhs, *)
    }
}

impl Div for K {
    type Output = KResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(x), Self::Int(y)) => Ok(K::Float(x as f64 / y as f64)),
            (Self::Int(x), Self::Float(y)) => Ok(K::Float(x as f64 / y)),
            (Self::Int(x), Self::IntList(y)) => {
                let x = x as f64;
                Ok(K::FloatList(y.iter().map(|&e| x / e as f64).collect()))
            }
            (Self::Int(x), Self::FloatList(y)) => {
                let x = x as f64;
                Ok(K::FloatList(y.iter().map(|e| x / e).collect()))
            }
            (Self::Int(x), Self::GenList(y)) => Ok(K::GenList(
                y.into_iter()
                    .map(|e| K::Float(x as f64) / e)
                    .collect::<Result<_, _>>()?,
            )),

            (Self::Float(x), Self::Int(y)) => Ok(K::Float(x / y as f64)),
            (Self::Float(x), Self::Float(y)) => Ok(K::Float(x / y)),
            (Self::Float(x), Self::IntList(y)) => {
                Ok(K::FloatList(y.iter().map(|&e| x / e as f64).collect()))
            }
            (Self::Float(x), Self::FloatList(y)) => {
                Ok(K::FloatList(y.iter().map(|e| x / e).collect()))
            }
            (Self::Float(x), Self::GenList(y)) => Ok(K::GenList(
                y.into_iter()
                    .map(|e| K::Float(x) / e)
                    .collect::<Result<_, _>>()?,
            )),

            (Self::IntList(x), Self::Int(y)) => {
                let y = y as f64;
                Ok(K::FloatList(x.iter().map(|&e| e as f64 / y).collect()))
            }
            (Self::IntList(x), Self::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|&e| e as f64 / y).collect()))
            }
            (Self::IntList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&x, y)| x as f64 / y as f64).collect(),
                    ))
                }
            }
            (Self::IntList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&x, y)| x as f64 / y).collect(),
                    ))
                }
            }
            (Self::IntList(x), Self::GenList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::GenList(
                        y.into_iter()
                            .zip(x)
                            .map(|(y, x)| K::Float(x as f64) / y)
                            .collect::<Result<_, _>>()?,
                    ))
                }
            }

            (Self::FloatList(x), Self::Int(y)) => {
                let y = y as f64;
                Ok(K::FloatList(x.iter().map(|e| e / y).collect()))
            }
            (Self::FloatList(x), Self::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|e| e / y).collect()))
            }
            (Self::FloatList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(x, y)| x / y as f64).collect(),
                    ))
                }
            }
            (Self::FloatList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(x.iter().zip(y).map(|(&x, y)| x / y).collect()))
                }
            }
            (Self::FloatList(x), Self::GenList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::GenList(
                        y.into_iter()
                            .zip(x)
                            .map(|(y, x)| K::Float(x) / y)
                            .collect::<Result<_, _>>()?,
                    ))
                }
            }

            (Self::GenList(x), Self::Int(y)) => Ok(K::GenList(
                x.into_iter()
                    .map(|e| e / K::Float(y as f64))
                    .collect::<Result<_, _>>()?,
            )),
            (Self::GenList(x), Self::Float(y)) => Ok(K::GenList(
                x.into_iter()
                    .map(|e| e / K::Float(y))
                    .collect::<Result<_, _>>()?,
            )),
            (Self::GenList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::GenList(
                        x.into_iter()
                            .zip(y)
                            .map(|(x, y)| x / K::Float(y as f64))
                            .collect::<Result<_, _>>()?,
                    ))
                }
            }
            (Self::GenList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::GenList(
                        x.into_iter()
                            .zip(y)
                            .map(|(x, y)| x / K::Float(y))
                            .collect::<Result<_, _>>()?,
                    ))
                }
            }
            (Self::GenList(x), Self::GenList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::GenList(
                        x.into_iter()
                            .zip(y)
                            .map(|(x, y)| x / y)
                            .collect::<Result<_, _>>()?,
                    ))
                }
            }

            (_, _) => Err(RuntimeErrorCode::Type),
        }
    }
}
