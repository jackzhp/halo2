//! Gadgets and chips for elliptic curve operations.

use std::fmt;

use crate::{
    arithmetic::CurveAffine,
    circuit::{Chip, Layouter},
    plonk::Error,
};

/// The set of circuit instructions required to use the ECC gadgets.
pub trait EccInstructions<C: CurveAffine>: Chip<Field = C::Base> {
    /// Variable representing an element of the elliptic curve's scalar field.
    type Scalar: Clone + fmt::Debug;
    /// Variable representing an elliptic curve point.
    type Point: Clone + fmt::Debug;
    /// Variable representing a fixed elliptic curve point (constant in the circuit).
    type FixedPoint: Clone + fmt::Debug;

    /// Performs point addition, returning `a + b`.
    fn add(
        layouter: &mut impl Layouter<Self>,
        a: &Self::Point,
        b: &Self::Point,
    ) -> Result<Self::Point, Error>;

    /// Performs point doubling, returning `[2] a`.
    fn double(layouter: &mut impl Layouter<Self>, a: &Self::Point) -> Result<Self::Point, Error>;

    /// Performs variable-base scalar multiplication, returning `[scalar] base`.
    fn mul(
        layouter: &mut impl Layouter<Self>,
        scalar: &Self::Scalar,
        base: &Self::Point,
    ) -> Result<Self::Point, Error>;

    /// Performs fixed-base scalar multiplication, returning `[scalar] base`.
    fn mul_fixed(
        layouter: &mut impl Layouter<Self>,
        scalar: &Self::Scalar,
        base: &Self::FixedPoint,
    ) -> Result<Self::Point, Error>;
}

/// An element of the given elliptic curve's scalar field.
#[derive(Debug)]
pub struct Scalar<C: CurveAffine, EccChip: EccInstructions<C>> {
    inner: EccChip::Scalar,
}

/// An elliptic curve point over the given curve.
#[derive(Debug)]
pub struct Point<C: CurveAffine, EccChip: EccInstructions<C>> {
    inner: EccChip::Point,
}

impl<C: CurveAffine, EccChip: EccInstructions<C>> Point<C, EccChip> {
    /// Returns `self + other`.
    pub fn add(&self, mut layouter: impl Layouter<EccChip>, other: &Self) -> Result<Self, Error> {
        EccChip::add(&mut layouter, &self.inner, &other.inner).map(|inner| Point { inner })
    }

    /// Returns `[2] self`.
    pub fn double(&self, mut layouter: impl Layouter<EccChip>) -> Result<Self, Error> {
        EccChip::double(&mut layouter, &self.inner).map(|inner| Point { inner })
    }

    /// Returns `[by] self`.
    pub fn mul(
        &self,
        mut layouter: impl Layouter<EccChip>,
        by: &Scalar<C, EccChip>,
    ) -> Result<Self, Error> {
        EccChip::mul(&mut layouter, &by.inner, &self.inner).map(|inner| Point { inner })
    }
}

/// A constant elliptic curve point over the given curve, for which scalar multiplication
/// is more efficient.
#[derive(Debug)]
pub struct FixedPoint<C: CurveAffine, EccChip: EccInstructions<C>> {
    inner: EccChip::FixedPoint,
}

impl<C: CurveAffine, EccChip: EccInstructions<C>> FixedPoint<C, EccChip> {
    /// Returns `[by] self`.
    pub fn mul(
        &self,
        mut layouter: impl Layouter<EccChip>,
        by: &Scalar<C, EccChip>,
    ) -> Result<Point<C, EccChip>, Error> {
        EccChip::mul_fixed(&mut layouter, &by.inner, &self.inner).map(|inner| Point { inner })
    }
}
