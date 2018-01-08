use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

use cgmath::num_traits::Zero;

#[derive(Clone, Copy, Debug)]
pub struct Sample<T> {
    value: T,
    probability: f32,
}

impl<T> Sample<T> {
    pub fn new(value: T, probability: f32) -> Sample<T> {
        Sample { value, probability }
    }
}

impl<T> Default for Sample<T>
where
    T: Default,
{
    fn default() -> Sample<T> {
        T::default().into()
    }
}

impl<T> From<T> for Sample<T> {
    fn from(value: T) -> Sample<T> {
        Sample::new(value, 1.0)
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(suspicious_arithmetic_impl))]
impl<T, U, V> Add<Sample<U>> for Sample<T>
where
    T: Add<U, Output = V>,
{
    type Output = Sample<V>;
    fn add(self, rhs: Sample<U>) -> Sample<V> {
        Sample {
            value: self.value + rhs.value,
            probability: self.probability * rhs.probability,
        }
    }
}

impl<T, U> AddAssign<Sample<U>> for Sample<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, rhs: Sample<U>) {
        self.value += rhs.value;
        self.probability *= rhs.probability;
    }
}

impl<T, U, V> Mul<Sample<U>> for Sample<T>
where
    T: Mul<U, Output = V>,
{
    type Output = Sample<V>;
    fn mul(self, rhs: Sample<U>) -> Sample<V> {
        Sample {
            value: self.value * rhs.value,
            probability: self.probability * rhs.probability,
        }
    }
}

impl<T, U> MulAssign<Sample<U>> for Sample<T>
where
    T: MulAssign<U>,
{
    fn mul_assign(&mut self, rhs: Sample<U>) {
        self.value *= rhs.value;
        self.probability *= rhs.probability;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Estimator<T> {
    value: T,
    n: u32,
}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default))]
impl<T> Estimator<T> {
    pub fn new() -> Estimator<T>
    where
        T: Zero,
    {
        Estimator {
            value: T::zero(),
            n: 0,
        }
    }

    pub fn add(&mut self, sample: Sample<T>)
    where
        T: AddAssign<T> + Div<f32, Output = T>,
    {
        self.value += sample.value / sample.probability;
        self.n += 1;
    }

    pub fn value(self) -> T
    where
        T: Div<f32, Output = T>,
    {
        assert!(self.n != 0);
        self.value / (self.n as f32)
    }
}

impl<T> Default for Estimator<T>
where
    T: Zero,
{
    fn default() -> Estimator<T> {
        Estimator::new()
    }
}
