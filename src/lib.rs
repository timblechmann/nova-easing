// Copyright (C) 2025 Tim Blechmann
// SPDX-License-Identifier: MIT

#![cfg_attr(feature = "nightly", feature(portable_simd))]

use core::ops::*;
use num_traits::{Float, FromPrimitive};

#[cfg(feature = "nightly")]
use core::simd::{LaneCount, Mask, Simd, SupportedLaneCount};

#[cfg(feature = "nightly")]
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};

#[cfg(feature = "nightly")]
use std::simd::num::SimdFloat;

#[cfg(feature = "nightly")]
use std::simd::StdFloat;

////////////////////////////////////////////////////////////////////////////////////////////////////

// Marker trait for scalar float types we support.
trait Scalar: Float + FromPrimitive {}
impl Scalar for f32 {}
impl Scalar for f64 {}

mod internal {
    pub trait Sealed {}

    pub trait CurveParam<T>: Sealed + Copy {
        fn to_curve(self) -> T;
    }

    #[cfg(feature = "nightly")]
    pub trait SimdScalar: core::simd::SimdElement + Copy {
        fn from_f32_scalar(val: f32) -> Self;
        fn ln_2() -> Self;
    }
}

impl internal::CurveParam<f32> for f32 {
    fn to_curve(self) -> f32 {
        self
    }
}

impl internal::CurveParam<f64> for f64 {
    fn to_curve(self) -> f64 {
        self
    }
}

#[cfg(feature = "nightly")]
impl internal::SimdScalar for f32 {
    fn from_f32_scalar(val: f32) -> Self {
        val
    }
    fn ln_2() -> Self {
        2.0f32.ln()
    }
}

#[cfg(feature = "nightly")]
impl internal::SimdScalar for f64 {
    fn from_f32_scalar(val: f32) -> Self {
        val as f64
    }
    fn ln_2() -> Self {
        2.0f64.ln()
    }
}

#[cfg(feature = "nightly")]
impl<const N: usize> internal::CurveParam<Simd<f32, N>> for f32
where
    LaneCount<N>: SupportedLaneCount,
    Simd<f32, N>: EasingImplHelper,
{
    fn to_curve(self) -> Simd<f32, N> {
        Simd::splat(self)
    }
}

#[cfg(feature = "nightly")]
impl<const N: usize> internal::CurveParam<Simd<f32, N>> for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
    Simd<f32, N>: EasingImplHelper,
{
    fn to_curve(self) -> Simd<f32, N> {
        self
    }
}

#[cfg(feature = "nightly")]
impl<const N: usize> internal::CurveParam<Simd<f64, N>> for f64
where
    LaneCount<N>: SupportedLaneCount,
    Simd<f64, N>: EasingImplHelper,
{
    fn to_curve(self) -> Simd<f64, N> {
        Simd::splat(self)
    }
}

#[cfg(feature = "nightly")]
impl<const N: usize> internal::CurveParam<Simd<f64, N>> for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
    Simd<f64, N>: EasingImplHelper,
{
    fn to_curve(self) -> Simd<f64, N> {
        self
    }
}

/// A trait providing easing functions for smooth interpolation.
///
/// Easing functions take a value `t` in the range [0, 1] and return an eased value
/// in the same range, useful for animations and transitions.
///
/// Supported for scalar types (`f32`, `f64`) and SIMD vectors (with `nightly` feature).
/// See [easings.net](https://easings.net/) for visualizations.
pub trait EasingArgument: internal::Sealed + Sized + Copy {
    /// Applies quadratic easing in. Starts slow and accelerates.
    ///
    /// See [easings.net](https://easings.net/#easeInQuad) for visualization.
    #[allow(private_bounds)]
    fn ease_in_quad(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_in_pow(2)
    }

    /// Applies quadratic easing out. Starts fast and decelerates.
    ///
    /// See [easings.net](https://easings.net/#easeOutQuad) for visualization.
    #[allow(private_bounds)]
    fn ease_out_quad(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_out_pow(2)
    }

    /// Applies quadratic easing in-out. Accelerates then decelerates.
    ///
    /// See [easings.net](https://easings.net/#easeInOutQuad) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_quad(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_quad(self)
    }

    /// Applies cubic easing in. Starts slow and accelerates more gradually.
    ///
    /// See [easings.net](https://easings.net/#easeInCubic) for visualization.
    #[allow(private_bounds)]
    fn ease_in_cubic(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_in_pow(3)
    }

    /// Applies cubic easing out. Starts fast and decelerates more gradually.
    ///
    /// See [easings.net](https://easings.net/#easeOutCubic) for visualization.
    #[allow(private_bounds)]
    fn ease_out_cubic(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_out_pow(3)
    }

    /// Applies cubic easing in-out. Accelerates then decelerates more gradually.
    ///
    /// See [easings.net](https://easings.net/#easeInOutCubic) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_cubic(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_cubic(self)
    }

    /// Applies quartic easing in. Starts very slow and accelerates sharply.
    ///
    /// See [easings.net](https://easings.net/#easeInQuart) for visualization.
    #[allow(private_bounds)]
    fn ease_in_quart(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_in_pow(4)
    }

    /// Applies quartic easing out. Starts very fast and decelerates sharply.
    ///
    /// See [easings.net](https://easings.net/#easeOutQuart) for visualization.
    #[allow(private_bounds)]
    fn ease_out_quart(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_out_pow(4)
    }

    /// Applies quartic easing in-out. Accelerates sharply then decelerates sharply.
    ///
    /// See [easings.net](https://easings.net/#easeInOutQuart) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_quart(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_quart(self)
    }

    /// Applies quintic easing in. Starts extremely slow and accelerates very sharply.
    ///
    /// See [easings.net](https://easings.net/#easeInQuint) for visualization.
    #[allow(private_bounds)]
    fn ease_in_quint(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_in_pow(5)
    }

    /// Applies quintic easing out. Starts extremely fast and decelerates very sharply.
    ///
    /// See [easings.net](https://easings.net/#easeOutQuint) for visualization.
    #[allow(private_bounds)]
    fn ease_out_quint(self) -> Self
    where
        Self: EasingImplHelper,
    {
        self.ease_out_pow(5)
    }

    /// Applies quintic easing in-out. Accelerates very sharply then decelerates very sharply.
    ///
    /// See [easings.net](https://easings.net/#easeInOutQuint) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_quint(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_quint(self)
    }

    /// Applies back easing in-out. Accelerates with overshoot then decelerates with overshoot.
    ///
    /// See [easings.net](https://easings.net/#easeInOutBack) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_back(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_back(self)
    }

    /// Applies bounce easing in. Starts with bounces and settles.
    ///
    /// See [easings.net](https://easings.net/#easeInBounce) for visualization.
    #[allow(private_bounds)]
    fn ease_in_bounce(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let one = Self::from_f32(1.0);
        one - <Self as EasingImplHelper>::ease_out_bounce(one - self)
    }

    /// Applies bounce easing out. Ends with bounces.
    ///
    /// See [easings.net](https://easings.net/#easeOutBounce) for visualization.
    #[allow(private_bounds)]
    fn ease_out_bounce(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_out_bounce(self)
    }

    /// Applies bounce easing in-out. Bounces at start and end.
    ///
    /// See [easings.net](https://easings.net/#easeInOutBounce) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_bounce(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_bounce(self)
    }

    /// Applies exponential easing in. Starts very slow and accelerates exponentially.
    ///
    /// See [easings.net](https://easings.net/#easeInExpo) for visualization.
    #[allow(private_bounds)]
    fn ease_in_expo(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_expo(self)
    }

    /// Applies exponential easing out. Starts very fast and decelerates exponentially.
    ///
    /// See [easings.net](https://easings.net/#easeOutExpo) for visualization.
    #[allow(private_bounds)]
    fn ease_out_expo(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_out_expo(self)
    }

    /// Applies exponential easing in-out. Accelerates exponentially then decelerates exponentially.
    ///
    /// See [easings.net](https://easings.net/#easeInOutExpo) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_expo(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_expo(self)
    }

    /// Applies elastic easing in. Starts with oscillation and settles.
    ///
    /// See [easings.net](https://easings.net/#easeInElastic) for visualization.
    #[allow(private_bounds)]
    fn ease_in_elastic(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_elastic(self)
    }

    /// Applies elastic easing out. Ends with oscillation.
    ///
    /// See [easings.net](https://easings.net/#easeOutElastic) for visualization.
    #[allow(private_bounds)]
    fn ease_out_elastic(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_out_elastic(self)
    }

    /// Applies elastic easing in-out. Oscillates at start and end.
    ///
    /// See [easings.net](https://easings.net/#easeInOutElastic) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_elastic(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_elastic(self)
    }

    /// Applies sine easing in. Starts slow with a smooth curve.
    ///
    /// See [easings.net](https://easings.net/#easeInSine) for visualization.
    #[allow(private_bounds)]
    fn ease_in_sine(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let one = Self::from_f32(1.0);
        let pi_half = Self::from_f32(std::f32::consts::FRAC_PI_2);
        one - (self * pi_half).cos()
    }

    /// Applies sine easing out. Ends slow with a smooth curve.
    ///
    /// See [easings.net](https://easings.net/#easeOutSine) for visualization.
    #[allow(private_bounds)]
    fn ease_out_sine(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let pi_half = Self::from_f32(std::f32::consts::FRAC_PI_2);
        (self * pi_half).sin()
    }

    /// Applies sine easing in-out. Smooth acceleration and deceleration.
    ///
    /// See [easings.net](https://easings.net/#easeInOutSine) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_sine(self) -> Self
    where
        Self: EasingImplHelper,
    {
        use std::f32::consts::PI;
        let cos_val = (self * Self::from_f32(PI)).cos();
        cos_val.mul_add(Self::from_f32(-0.5), Self::from_f32(0.5))
    }

    /// Applies circular easing in. Starts very slow and accelerates sharply.
    ///
    /// See [easings.net](https://easings.net/#easeInCirc) for visualization.
    #[allow(private_bounds)]
    fn ease_in_circ(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let one = Self::from_f32(1.0);
        one - (one - self.powi(2)).sqrt()
    }

    /// Applies circular easing out. Starts very fast and decelerates sharply.
    ///
    /// See [easings.net](https://easings.net/#easeOutCirc) for visualization.
    #[allow(private_bounds)]
    fn ease_out_circ(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let one = Self::from_f32(1.0);
        (one - (self - one).powi(2)).sqrt()
    }

    /// Applies circular easing in-out. Accelerates sharply then decelerates sharply.
    ///
    /// See [easings.net](https://easings.net/#easeInOutCirc) for visualization.
    #[allow(private_bounds)]
    fn ease_in_out_circ(self) -> Self
    where
        Self: EasingImplHelper,
    {
        <Self as EasingImplHelper>::ease_in_out_circ(self)
    }

    /// Applies back easing in. Starts with a slight overshoot.
    ///
    /// See [easings.net](https://easings.net/#easeInBack) for visualization.
    #[allow(private_bounds)]
    fn ease_in_back(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let c1 = Self::from_f32(1.70158);
        let c3 = Self::from_f32(2.70158);

        c3 * self.powi(3) - c1 * self.powi(2)
    }

    /// Applies back easing out. Ends with a slight overshoot.
    ///
    /// See [easings.net](https://easings.net/#easeOutBack) for visualization.
    #[allow(private_bounds)]
    fn ease_out_back(self) -> Self
    where
        Self: EasingImplHelper,
    {
        let c1 = Self::from_f32(1.70158);
        let c3 = Self::from_f32(2.70158);
        let one = Self::from_f32(1.0);

        one + c3 * (self - one).powi(3) + c1 * (self - one).powi(2)
    }

    /// Applies custom exponential easing in with a curve parameter.
    ///
    /// Accelerates from slow to fast using exponential growth controlled by the `curve` parameter.
    /// - `curve > 0`: Convex curve, steeper acceleration (e.g., `curve = 1.0` for moderate, `curve = 4.0` for sharp).
    /// - `curve < 0`: Concave curve, gentler acceleration (e.g., `curve = -1.0` for soft, `curve = -4.0` for very gradual).
    /// - `curve ≈ 0`: Approximates linear easing.
    ///
    /// The `curve` parameter can be a scalar or SIMD vector matching the easing argument type.
    /// Inspired by SuperCollider's `Env` curve parameter for envelope shaping.
    /// See [SuperCollider Env documentation](https://doc.sccode.org/Classes/Env.html) for more on curve values.
    #[allow(private_bounds)]
    fn ease_in_curve<C>(self, curve: C) -> Self
    where
        Self: EasingImplHelper,
        C: internal::CurveParam<Self>,
    {
        <Self as EasingImplHelper>::ease_in_curve(self, curve)
    }

    /// Applies custom exponential easing out with a curve parameter.
    ///
    /// Decelerates from fast to slow using exponential decay controlled by the `curve` parameter.
    /// - `curve > 0`: Convex curve, steeper deceleration.
    /// - `curve < 0`: Concave curve, gentler deceleration.
    /// - `curve ≈ 0`: Approximates linear easing.
    ///
    /// The `curve` parameter can be a scalar or SIMD vector matching the easing argument type.
    /// Mirrors `ease_in_curve` but in reverse. Inspired by SuperCollider's `Env` curve parameter.
    /// See [SuperCollider Env documentation](https://doc.sccode.org/Classes/Env.html).
    #[allow(private_bounds)]
    fn ease_out_curve<C>(self, curve: C) -> Self
    where
        Self: EasingImplHelper,
        C: internal::CurveParam<Self>,
    {
        <Self as EasingImplHelper>::ease_out_curve(self, curve)
    }

    /// Applies custom exponential easing in-out with a curve parameter.
    ///
    /// Accelerates then decelerates using exponential transitions controlled by the `curve` parameter.
    /// - `curve > 0`: Sharper acceleration and deceleration.
    /// - `curve < 0`: Softer transitions.
    /// - `curve ≈ 0`: Approximates linear easing.
    ///
    /// The `curve` parameter can be a scalar or SIMD vector matching the easing argument type.
    /// Combines `ease_in_curve` and `ease_out_curve` for smooth bidirectional transitions.
    /// Inspired by SuperCollider's `Env` curve parameter for envelope shaping.
    /// See [SuperCollider Env documentation](https://doc.sccode.org/Classes/Env.html).
    #[allow(private_bounds)]
    fn ease_in_out_curve<C>(self, curve: C) -> Self
    where
        Self: EasingImplHelper,
        C: internal::CurveParam<Self>,
    {
        <Self as EasingImplHelper>::ease_in_out_curve(self, curve)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait EasingImplHelper:
    Sub<Self, Output = Self>
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Sized
    + Copy
{
    fn from_f32(arg: f32) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn powi(self, n: i32) -> Self;
    #[allow(unused)]
    fn powf(self, other: Self) -> Self;
    fn double(self) -> Self {
        self + self
    }
    fn sqrt(self) -> Self;
    #[allow(unused)]
    fn exp(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;

    fn ease_in_pow(self, n: i32) -> Self {
        self.powi(n)
    }

    fn ease_out_pow(self, n: i32) -> Self {
        let one = Self::from_f32(1.0);
        one - (one - self).powi(n)
    }

    fn ease_in_out_quad(self) -> Self;
    fn ease_in_out_cubic(self) -> Self;
    fn ease_in_out_quart(self) -> Self;
    fn ease_in_out_quint(self) -> Self;
    fn ease_in_out_back(self) -> Self;
    fn ease_out_bounce(self) -> Self;
    fn ease_in_out_bounce(self) -> Self;
    fn ease_in_expo(self) -> Self;
    fn ease_out_expo(self) -> Self;
    fn ease_in_out_expo(self) -> Self;
    fn ease_in_elastic(self) -> Self;
    fn ease_out_elastic(self) -> Self;
    fn ease_in_out_elastic(self) -> Self;
    fn ease_in_out_circ(self) -> Self;

    fn ease_in_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>;
    fn ease_out_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>;
    fn ease_in_out_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: EasingImplHelper> internal::Sealed for T {}
impl<T: EasingImplHelper> EasingArgument for T {}

impl<T> EasingImplHelper for T
where
    T: Scalar,
{
    fn from_f32(arg: f32) -> Self {
        T::from(arg).unwrap()
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, other: Self) -> Self {
        self.powf(other)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    fn ease_in_out_quad(self) -> Self {
        let half = T::from(0.5).unwrap();
        let one = T::one();
        let two = T::from(2.0).unwrap();
        if self < half {
            two * self.powi(2)
        } else {
            one - ((two * self - two).powi(2) * half)
        }
    }
    fn ease_in_out_cubic(self) -> Self {
        let half = T::from(0.5).unwrap();
        if self < half {
            let cubed = self.powi(3);
            let doubled = cubed.double();
            doubled + doubled
        } else {
            let one = T::one();
            let two = T::from(2.0).unwrap();
            one - (two - self.double()).powi(3) * half
        }
    }
    fn ease_in_out_quart(self) -> Self {
        let half = T::from(0.5).unwrap();
        if self < half {
            T::from(8.0).unwrap() * self.powi(4)
        } else {
            let one = T::one();
            let two = T::from(2.0).unwrap();
            one - (two - self.double()).powi(4) * half
        }
    }
    fn ease_in_out_quint(self) -> Self {
        let half = T::from(0.5).unwrap();
        if self < half {
            T::from(16.0).unwrap() * self.powi(5)
        } else {
            let one = T::one();
            let two = T::from(2.0).unwrap();
            one - (two - self.double()).powi(5) * half
        }
    }
    fn ease_in_out_back(self) -> Self {
        let c2 = T::from(1.70158 * 1.525).unwrap();
        let half = T::from(0.5).unwrap();
        let two = T::from(2.0).unwrap();
        if self < half {
            let two_x = self.double();
            let pow_two_x_2 = two_x.powi(2);
            let inner = (c2 + T::one()).mul_add(two_x, -c2);
            pow_two_x_2 * inner * half
        } else {
            let two_x_minus_2 = self.double() - two;
            let pow_two_x_minus_2_2 = two_x_minus_2.powi(2);
            let inner = (c2 + T::one()).mul_add(self.double() - two, c2);
            pow_two_x_minus_2_2.mul_add(inner, two) * half
        }
    }
    fn ease_out_bounce(self) -> Self {
        let n1 = T::from(7.5625).unwrap();
        let one_over_d1 = T::from(1.0 / 2.75).unwrap();
        let two_over_d1 = T::from(2.0 / 2.75).unwrap();
        let two_point_five_over_d1 = T::from(2.5 / 2.75).unwrap();
        if self < one_over_d1 {
            n1 * self * self
        } else if self < two_over_d1 {
            let adjusted = self - T::from(1.5 / 2.75).unwrap();
            (adjusted * adjusted).mul_add(n1, T::from(0.75).unwrap())
        } else if self < two_point_five_over_d1 {
            let adjusted = self - T::from(2.25 / 2.75).unwrap();
            (adjusted * adjusted).mul_add(n1, T::from(0.9375).unwrap())
        } else {
            let adjusted = self - T::from(2.625 / 2.75).unwrap();
            (adjusted * adjusted).mul_add(n1, T::from(0.984375).unwrap())
        }
    }
    fn ease_in_out_bounce(self) -> Self {
        let half = T::from(0.5).unwrap();
        let one = T::one();
        if self < half {
            (one - EasingArgument::ease_out_bounce(one - self.double())) * half
        } else {
            (one + EasingArgument::ease_out_bounce(self.double() - one)) * half
        }
    }
    fn ease_in_expo(self) -> Self {
        if self == T::zero() {
            T::zero()
        } else {
            T::from(2.0).unwrap().powf(
                T::from(10.0)
                    .unwrap()
                    .mul_add(self, -T::from(10.0).unwrap()),
            )
        }
    }
    fn ease_out_expo(self) -> Self {
        if self == T::one() {
            T::one()
        } else {
            T::from(2.0)
                .unwrap()
                .powf(-T::from(10.0).unwrap() * self)
                .mul_add(-T::one(), T::one())
        }
    }
    fn ease_in_out_expo(self) -> Self {
        if self == T::zero() {
            T::zero()
        } else if self == T::one() {
            T::one()
        } else if self < T::from(0.5).unwrap() {
            T::from(2.0)
                .unwrap()
                .powf(
                    T::from(20.0)
                        .unwrap()
                        .mul_add(self, -T::from(10.0).unwrap()),
                )
                .mul_add(T::from(0.5).unwrap(), T::zero())
        } else {
            T::from(2.0)
                .unwrap()
                .powf(
                    T::from(-20.0)
                        .unwrap()
                        .mul_add(self, T::from(10.0).unwrap()),
                )
                .mul_add(-T::from(0.5).unwrap(), T::one())
        }
    }
    fn ease_in_elastic(self) -> Self {
        if self == T::zero() {
            T::zero()
        } else if self == T::one() {
            T::one()
        } else {
            let c4 = T::from(2.094_395_2).unwrap();
            -T::from(2.0)
                .unwrap()
                .powf(T::from(10.0).unwrap() * self - T::from(10.0).unwrap())
                * (self.mul_add(T::from(10.0).unwrap(), -T::from(10.75).unwrap()) * c4).sin()
        }
    }
    fn ease_out_elastic(self) -> Self {
        if self == T::zero() {
            T::zero()
        } else if self == T::one() {
            T::one()
        } else {
            let c4 = T::from(2.094_395_2).unwrap();
            T::from(2.0)
                .unwrap()
                .powf(-T::from(10.0).unwrap() * self)
                .mul_add(
                    (self.mul_add(T::from(10.0).unwrap(), -T::from(0.75).unwrap()) * c4).sin(),
                    T::one(),
                )
        }
    }
    fn ease_in_out_elastic(self) -> Self {
        if self == T::zero() {
            T::zero()
        } else if self == T::one() {
            T::one()
        } else if self < T::from(0.5).unwrap() {
            let c5 = T::from(1.396_263_4).unwrap();
            -T::from(2.0)
                .unwrap()
                .powf(T::from(20.0).unwrap() * self - T::from(10.0).unwrap())
                * (self.mul_add(T::from(20.0).unwrap(), -T::from(11.125).unwrap()) * c5).sin()
                * T::from(0.5).unwrap()
        } else {
            let c5 = T::from(1.396_263_4).unwrap();
            T::from(2.0)
                .unwrap()
                .powf(-T::from(20.0).unwrap() * self + T::from(10.0).unwrap())
                .mul_add(
                    (self.mul_add(T::from(20.0).unwrap(), -T::from(11.125).unwrap()) * c5).sin()
                        * T::from(0.5).unwrap(),
                    T::one(),
                )
        }
    }
    fn ease_in_out_circ(self) -> Self {
        let half = T::from(0.5).unwrap();
        let one = T::one();
        let two = T::from(2.0).unwrap();
        let double = self.double();
        if self < half {
            (one - (one - double.powi(2)).sqrt()) * half
        } else {
            ((one - (two - double).powi(2)).sqrt() + one) * half
        }
    }

    fn ease_in_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>,
    {
        let c = curve.to_curve();
        if c.abs() < T::from(0.001).unwrap() {
            self
        } else {
            let grow = c.exp();
            let one = T::one();
            let a = one / (one - grow);
            a - (a * grow.powf(self))
        }
    }

    fn ease_out_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>,
    {
        let one = T::one();
        one - <Self as EasingImplHelper>::ease_in_curve(one - self, curve)
    }

    fn ease_in_out_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>,
    {
        let half = T::from(0.5).unwrap();
        if self < half {
            <Self as EasingImplHelper>::ease_in_curve(self.double(), curve) * half
        } else {
            half + <Self as EasingImplHelper>::ease_out_curve((self - half).double(), curve) * half
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "nightly")]
impl<T, const N: usize> EasingImplHelper for Simd<T, N>
where
    T: internal::SimdScalar + core::simd::SimdElement,
    T::Mask: core::simd::MaskElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: StdFloat
        + SimdFloat
        + SimdPartialEq<Mask = Mask<T::Mask, N>>
        + SimdPartialOrd
        + Add<Output = Simd<T, N>>
        + Sub<Output = Simd<T, N>>
        + Mul<Output = Simd<T, N>>
        + Div<Output = Simd<T, N>>
        + Neg<Output = Simd<T, N>>,
{
    fn from_f32(arg: f32) -> Self {
        Simd::splat(T::from_f32_scalar(arg))
    }

    fn sin(self) -> Self {
        <Self as StdFloat>::sin(self)
    }

    fn cos(self) -> Self {
        <Self as StdFloat>::cos(self)
    }

    fn powi(self, n: i32) -> Self {
        if n == 1 {
            self
        } else if n % 2 == 0 {
            let tmp = self.powi(n / 2);
            tmp * tmp
        } else {
            self * self.powi(n - 1)
        }
    }

    fn powf(self, other: Self) -> Self {
        <Self as StdFloat>::exp(other * <Self as StdFloat>::ln(self))
    }

    fn sqrt(self) -> Self {
        <Self as StdFloat>::sqrt(self)
    }

    fn exp(self) -> Self {
        <Self as StdFloat>::exp(self)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        <Self as StdFloat>::mul_add(self, a, b)
    }

    fn ease_in_out_quad(self) -> Self {
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);

        let lower_half = self.powi(2).double();
        let upper_half = Self::from_f32(1.0) - (self.double() - Self::from_f32(2.0)).powi(2) * half;

        mask.select(lower_half, upper_half)
    }

    fn ease_in_out_cubic(self) -> Self {
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);

        let lower_half = {
            let cubed = self.powi(3);
            let doubled = cubed.double();
            doubled + doubled
        };

        let upper_half = {
            let one = Self::from_f32(1.0);
            let two = Self::from_f32(2.0);
            one - (two - self.double()).powi(3) * half
        };

        mask.select(lower_half, upper_half)
    }

    fn ease_in_out_quart(self) -> Self {
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);

        let lower_half = { Self::from_f32(8.0) * self.powi(4) };
        let upper_half = {
            let one = Self::from_f32(1.0);
            let two = Self::from_f32(2.0);
            one - (two - self.double()).powi(4) * half
        };
        mask.select(lower_half, upper_half)
    }

    fn ease_in_out_quint(self) -> Self {
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);

        let lower_half = { Self::from_f32(16.0) * self.powi(5) };
        let upper_half = {
            let one = Self::from_f32(1.0);
            let two = Self::from_f32(2.0);
            one - (two - self.double()).powi(5) * half
        };
        mask.select(lower_half, upper_half)
    }

    fn ease_in_out_back(self) -> Self {
        let c2 = Self::from_f32(1.70158 * 1.525);
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);

        let lower_half = {
            let two_x = self.double();
            let pow_two_x_2 = two_x.powi(2);
            let inner = StdFloat::mul_add(c2 + Self::from_f32(1.0), two_x, -c2);
            pow_two_x_2 * inner
        };
        let upper_half = {
            let two_x_minus_2 = self.double() - Self::from_f32(2.0);
            let pow_two_x_minus_2_2 = two_x_minus_2.powi(2);
            let inner = StdFloat::mul_add(
                c2 + Self::from_f32(1.0),
                self.double() - Self::from_f32(2.0),
                c2,
            );
            StdFloat::mul_add(pow_two_x_minus_2_2, inner, Self::from_f32(2.0))
        };
        mask.select(lower_half, upper_half) * half
    }

    fn ease_out_bounce(self) -> Self {
        let n1 = Self::from_f32(7.5625);
        let one_over_d1 = Self::from_f32(1.0 / 2.75);
        let two_over_d1 = Self::from_f32(2.0 / 2.75);
        let two_point_five_over_d1 = Self::from_f32(2.5 / 2.75);
        let mask1 = self.simd_lt(one_over_d1);
        let mask2 = self.simd_lt(two_over_d1);
        let mask3 = self.simd_lt(two_point_five_over_d1);
        let branch1 = n1 * self * self;
        let adjusted2 = self - Self::from_f32(1.5 / 2.75);
        let branch2 = StdFloat::mul_add(adjusted2 * adjusted2, n1, Self::from_f32(0.75));
        let adjusted3 = self - Self::from_f32(2.25 / 2.75);
        let branch3 = StdFloat::mul_add(adjusted3 * adjusted3, n1, Self::from_f32(0.9375));
        let adjusted4 = self - Self::from_f32(2.625 / 2.75);
        let branch4 = StdFloat::mul_add(adjusted4 * adjusted4, n1, Self::from_f32(0.984375));
        mask1.select(
            branch1,
            mask2.select(branch2, mask3.select(branch3, branch4)),
        )
    }

    fn ease_in_out_bounce(self) -> Self {
        let half = Self::from_f32(0.5);
        let one = Self::from_f32(1.0);
        let mask = self.simd_lt(half);
        let lower_half = one - EasingArgument::ease_out_bounce(one - self.double());
        let upper_half = one + EasingArgument::ease_out_bounce(self.double() - one);
        mask.select(lower_half, upper_half) * half
    }

    fn ease_in_expo(self) -> Self {
        let zero = Self::from_f32(0.0);
        let ln2 = Simd::splat(T::ln_2());
        let ten = Self::from_f32(10.0);
        let mask_zero = self.simd_eq(zero);
        let exponent = StdFloat::mul_add(ten, self, -ten);
        let normal = <Self as StdFloat>::exp(exponent * ln2);
        mask_zero.select(zero, normal)
    }

    fn ease_out_expo(self) -> Self {
        let one = Self::from_f32(1.0);
        let ln2 = Simd::splat(T::ln_2());
        let neg_ten = Self::from_f32(-10.0);
        let mask_one = self.simd_eq(one);
        let exponent = neg_ten * self;
        let normal = StdFloat::mul_add(
            <Self as StdFloat>::exp(exponent * ln2),
            -Self::from_f32(1.0),
            one,
        );
        mask_one.select(one, normal)
    }

    fn ease_in_out_expo(self) -> Self {
        let zero = Self::from_f32(0.0);
        let one = Self::from_f32(1.0);
        let half = Self::from_f32(0.5);
        let ln2 = Simd::splat(T::ln_2());
        let twenty = Self::from_f32(20.0);
        let ten = Self::from_f32(10.0);
        let mask_zero = self.simd_eq(zero);
        let mask_one = self.simd_eq(one);
        let mask_half = self.simd_lt(half);
        let exponent_lower = StdFloat::mul_add(twenty, self, -ten);
        let branch_lower = <Self as StdFloat>::exp(exponent_lower * ln2) * half;
        let exponent_upper = StdFloat::mul_add(-twenty, self, ten);
        let branch_upper =
            StdFloat::mul_add(<Self as StdFloat>::exp(exponent_upper * ln2), -half, one);
        let temp = mask_half.select(branch_lower, branch_upper);
        let temp2 = mask_one.select(one, temp);
        mask_zero.select(zero, temp2)
    }

    fn ease_in_elastic(self) -> Self {
        let zero = Self::from_f32(0.0);
        let one = Self::from_f32(1.0);
        let ln2 = Simd::splat(T::ln_2());
        let c4 = Self::from_f32(2.094_395_2);
        let ten = Self::from_f32(10.0);
        let minus_ten_point_75 = Self::from_f32(-10.75);
        let mask_zero = self.simd_eq(zero);
        let mask_one = self.simd_eq(one);
        let exponent = StdFloat::mul_add(ten, self, -ten);
        let sin_arg = StdFloat::mul_add(ten, self, minus_ten_point_75) * c4;
        let normal = -<Self as StdFloat>::exp(exponent * ln2) * <Self as StdFloat>::sin(sin_arg);
        let temp = mask_one.select(one, normal);
        mask_zero.select(zero, temp)
    }

    fn ease_out_elastic(self) -> Self {
        let zero = Self::from_f32(0.0);
        let one = Self::from_f32(1.0);
        let ln2 = Simd::splat(T::ln_2());
        let c4 = Self::from_f32(2.094_395_2);
        let ten = Self::from_f32(10.0);
        let minus_zero_point_75 = Self::from_f32(-0.75);
        let mask_zero = self.simd_eq(zero);
        let mask_one = self.simd_eq(one);
        let exponent = -ten * self;
        let sin_arg = StdFloat::mul_add(ten, self, minus_zero_point_75) * c4;
        let normal = StdFloat::mul_add(
            <Self as StdFloat>::exp(exponent * ln2),
            <Self as StdFloat>::sin(sin_arg),
            one,
        );
        let temp = mask_one.select(one, normal);
        mask_zero.select(zero, temp)
    }

    fn ease_in_out_elastic(self) -> Self {
        let zero = Self::from_f32(0.0);
        let one = Self::from_f32(1.0);
        let half = Self::from_f32(0.5);
        let ln2 = Simd::splat(T::ln_2());
        let c5 = Self::from_f32(1.396_263_4);
        let twenty = Self::from_f32(20.0);
        let ten = Self::from_f32(10.0);
        let minus_eleven_point_125 = Self::from_f32(-11.125);
        let mask_zero = self.simd_eq(zero);
        let mask_one = self.simd_eq(one);
        let mask_half = self.simd_lt(half);
        let exponent_lower = StdFloat::mul_add(twenty, self, -ten);
        let sin_arg = StdFloat::mul_add(twenty, self, minus_eleven_point_125) * c5;
        let branch_lower = -<Self as StdFloat>::exp(exponent_lower * ln2)
            * <Self as StdFloat>::sin(sin_arg)
            * half;
        let exponent_upper = StdFloat::mul_add(-twenty, self, ten);
        let branch_upper = StdFloat::mul_add(
            <Self as StdFloat>::exp(exponent_upper * ln2),
            <Self as StdFloat>::sin(sin_arg) * half,
            one,
        );
        let temp = mask_half.select(branch_lower, branch_upper);
        let temp2 = mask_one.select(one, temp);
        mask_zero.select(zero, temp2)
    }

    fn ease_in_out_circ(self) -> Self {
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);

        let one = Self::from_f32(1.0);
        let two = Self::from_f32(2.0);
        let double = self.double();

        let lower_half = one - StdFloat::sqrt(one - double.powi(2));
        let upper_half = StdFloat::sqrt(one - (two - double).powi(2)) + one;
        mask.select(lower_half, upper_half) * half
    }

    fn ease_in_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>,
    {
        let c = curve.to_curve();
        let abs_curve = SimdFloat::abs(c);
        let mask = abs_curve.simd_lt(Self::from_f32(0.001));
        let grow = <Self as StdFloat>::exp(c);
        let a = Self::from_f32(1.0) / (Self::from_f32(1.0) - grow);
        let normal = a - (a * grow.powf(self));
        mask.select(self, normal)
    }

    fn ease_out_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>,
    {
        let one = Self::from_f32(1.0);
        one - <Self as EasingImplHelper>::ease_in_curve(one - self, curve)
    }

    fn ease_in_out_curve<C>(self, curve: C) -> Self
    where
        C: internal::CurveParam<Self>,
    {
        let half = Self::from_f32(0.5);
        let mask = self.simd_lt(half);
        let lower_half = <Self as EasingImplHelper>::ease_in_curve(self.double(), curve) * half;
        let upper_half =
            half + <Self as EasingImplHelper>::ease_out_curve((self - half).double(), curve) * half;
        mask.select(lower_half, upper_half)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::EasingArgument;
    #[cfg(feature = "nightly")]
    use std::simd::{Simd, f32x4};

    #[cfg(feature = "nightly")]
    mod comparison_tests {
        use approx::assert_relative_eq;
        use paste::paste;

        macro_rules! generate_comparison_tests {
            ($func:ident) => {
                paste! {
                    #[test]
                    fn [<$func _f32_vs_f32x4>]() {
                        use super::EasingArgument;
                        let points = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
                        for &x in &points {
                            let scalar = EasingArgument::$func(x);
                            let vector = EasingArgument::$func(core::simd::f32x4::splat(x))[0];
                            assert_relative_eq!(scalar, vector, epsilon = 1e-6);
                        }
                    }
                }
            };
        }

        generate_comparison_tests!(ease_in_quad);
        generate_comparison_tests!(ease_out_quad);
        generate_comparison_tests!(ease_in_out_quad);
        generate_comparison_tests!(ease_in_cubic);
        generate_comparison_tests!(ease_out_cubic);
        generate_comparison_tests!(ease_in_out_cubic);
        generate_comparison_tests!(ease_in_quart);
        generate_comparison_tests!(ease_out_quart);
        generate_comparison_tests!(ease_in_out_quart);
        generate_comparison_tests!(ease_in_quint);
        generate_comparison_tests!(ease_out_quint);
        generate_comparison_tests!(ease_in_out_quint);
        generate_comparison_tests!(ease_in_sine);
        generate_comparison_tests!(ease_out_sine);
        generate_comparison_tests!(ease_in_out_sine);
        generate_comparison_tests!(ease_in_circ);
        generate_comparison_tests!(ease_out_circ);
        generate_comparison_tests!(ease_in_out_circ);
        generate_comparison_tests!(ease_in_back);
        generate_comparison_tests!(ease_out_back);
        generate_comparison_tests!(ease_in_out_back);
        generate_comparison_tests!(ease_in_bounce);
        generate_comparison_tests!(ease_out_bounce);
        generate_comparison_tests!(ease_in_out_bounce);
        generate_comparison_tests!(ease_in_expo);
        generate_comparison_tests!(ease_out_expo);
        generate_comparison_tests!(ease_in_out_expo);
        generate_comparison_tests!(ease_in_elastic);
        generate_comparison_tests!(ease_out_elastic);
        generate_comparison_tests!(ease_in_out_elastic);

        #[test]
        fn ease_in_curve_f32_vs_f32x4() {
            use super::EasingArgument;
            let points = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
            for &x in &points {
                let scalar = EasingArgument::ease_in_curve(x, 1.0f32);
                let vector = EasingArgument::ease_in_curve(core::simd::f32x4::splat(x), 1.0f32)[0];
                assert_relative_eq!(scalar, vector, epsilon = 1e-6);
            }
        }

        #[test]
        fn ease_out_curve_f32_vs_f32x4() {
            use super::EasingArgument;
            let points = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
            for &x in &points {
                let scalar = EasingArgument::ease_out_curve(x, 1.0f32);
                let vector = EasingArgument::ease_out_curve(core::simd::f32x4::splat(x), 1.0f32)[0];
                assert_relative_eq!(scalar, vector, epsilon = 1e-6);
            }
        }

        #[test]
        fn ease_in_out_curve_f32_vs_f32x4() {
            use super::EasingArgument;
            let points = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
            for &x in &points {
                let scalar = EasingArgument::ease_in_out_curve(x, 1.0f32);
                let vector =
                    EasingArgument::ease_in_out_curve(core::simd::f32x4::splat(x), 1.0f32)[0];
                assert_relative_eq!(scalar, vector, epsilon = 1e-6);
            }
        }
    }

    mod boundary_and_symmetry_tests {
        use super::EasingArgument;
        use approx::assert_relative_eq;
        use paste::paste;

        // Boundary tests: f(0) == 0 and f(1) == 1 for all functions
        macro_rules! generate_boundary_tests {
            ($type:ty, $epsilon:expr) => {
                paste! {
                    #[test]
                    fn [<boundary_tests_ $type>]() {
                        let zero: $type = 0.0.into();
                        let one: $type = 1.0.into();

                        assert_relative_eq!(zero.ease_in_quad(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_quad(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_quad(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_quad(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_quad(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_quad(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_cubic(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_cubic(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_cubic(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_cubic(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_cubic(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_cubic(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_quart(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_quart(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_quart(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_quart(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_quart(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_quart(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_quint(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_quint(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_quint(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_quint(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_quint(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_quint(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_sine(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_sine(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_sine(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_sine(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_sine(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_sine(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_circ(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_circ(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_circ(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_circ(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_circ(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_circ(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_back(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_back(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_back(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_back(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_back(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_back(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_bounce(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_bounce(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_bounce(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_bounce(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_bounce(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_bounce(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_expo(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_expo(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_expo(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_expo(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_expo(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_expo(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_elastic(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_elastic(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_elastic(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_elastic(), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_elastic(), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_elastic(), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_curve(1.0), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_curve(1.0), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_curve(-1.0), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_curve(-1.0), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_out_curve(1.0), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_curve(1.0), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_out_curve(-1.0), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_out_curve(-1.0), one, epsilon = $epsilon);

                        assert_relative_eq!(zero.ease_in_out_curve(1.0), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_curve(1.0), one, epsilon = $epsilon);
                        assert_relative_eq!(zero.ease_in_out_curve(-1.0), zero, epsilon = $epsilon);
                        assert_relative_eq!(one.ease_in_out_curve(-1.0), one, epsilon = $epsilon);
                     }
                }
            };
        }

        // Mirror symmetry: ease_out(t) == 1 - ease_in(1 - t)
        macro_rules! generate_mirror_symmetry_tests {
            ($type:ty, $epsilon:expr) => {
                paste! {
                    #[test]
                    fn [<mirror_symmetry_ $type>]() {
                        let points = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
                        let one: $type = 1.0.into();
                        for &t in &points {
                            let t_val: $type = t.into();
                            let one_minus_t: $type = (1.0 - t).into();

                            assert_relative_eq!(t_val.ease_out_quad(), one - one_minus_t.ease_in_quad(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_cubic(), one - one_minus_t.ease_in_cubic(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_quart(), one - one_minus_t.ease_in_quart(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_quint(), one - one_minus_t.ease_in_quint(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_sine(), one - one_minus_t.ease_in_sine(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_circ(), one - one_minus_t.ease_in_circ(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_back(), one - one_minus_t.ease_in_back(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_bounce(), one - one_minus_t.ease_in_bounce(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_expo(), one - one_minus_t.ease_in_expo(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_elastic(), one - one_minus_t.ease_in_elastic(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_out_curve(1.0), one - one_minus_t.ease_in_curve(1.0), epsilon = $epsilon);
                        }
                    }
                }
            };
        }

        // In-out symmetry: ease_in_out(t) == 1 - ease_in_out(1 - t)
        macro_rules! generate_in_out_symmetry_tests {
            ($type:ty, $epsilon:expr) => {
                paste! {
                    #[test]
                    fn [<in_out_symmetry_ $type>]() {
                        let points = [0.1, 0.2, 0.3, 0.4, 0.5];
                        let one: $type = 1.0.into();
                        for &t in &points {
                            let t_val: $type = t.into();
                            let one_minus_t: $type = (1.0 - t).into();

                            assert_relative_eq!(t_val.ease_in_out_quad(), one - one_minus_t.ease_in_out_quad(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_cubic(), one - one_minus_t.ease_in_out_cubic(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_quart(), one - one_minus_t.ease_in_out_quart(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_quint(), one - one_minus_t.ease_in_out_quint(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_sine(), one - one_minus_t.ease_in_out_sine(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_circ(), one - one_minus_t.ease_in_out_circ(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_back(), one - one_minus_t.ease_in_out_back(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_bounce(), one - one_minus_t.ease_in_out_bounce(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_expo(), one - one_minus_t.ease_in_out_expo(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_elastic(), one - one_minus_t.ease_in_out_elastic(), epsilon = $epsilon);
                            assert_relative_eq!(t_val.ease_in_out_curve(1.0), one - one_minus_t.ease_in_out_curve(1.0), epsilon = $epsilon);
                        }
                    }
                }
            };
        }

        // Instantiate for f32
        generate_boundary_tests!(f32, 1e-6);
        generate_mirror_symmetry_tests!(f32, 1e-6);
        generate_in_out_symmetry_tests!(f32, 1e-6);

        // Instantiate for f64
        generate_boundary_tests!(f64, 1e-7);
        generate_mirror_symmetry_tests!(f64, 1e-7);
        generate_in_out_symmetry_tests!(f64, 1e-7);
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn test_mixed_arguments() {
        let arg: f32x4 = Simd::splat(0.5);
        {
            let curve = 1.0f32;
            arg.ease_in_out_curve(curve);
        }

        {
            let curve = f32x4::splat(1.0);
            arg.ease_in_out_curve(curve);
        }
    }
}

#[cfg(test)]
mod reference_value_tests {
    use super::EasingArgument;
    use approx::assert_relative_eq;

    macro_rules! generate_reference_tests {
        ($func:ident, $vals:expr) => {
            #[test]
            fn $func() {
                let inputs = [0.2f32, 0.4, 0.5, 0.6, 0.8];
                #[allow(clippy::approx_constant)]
                let expected = $vals;
                for (&input, &exp) in inputs.iter().zip(expected.iter()) {
                    assert_relative_eq!(input.$func(), exp, epsilon = 1e-6);
                }
            }
        };
        ($func:ident, $param:expr, $vals:expr) => {
            #[test]
            fn $func() {
                let inputs = [0.2f32, 0.4, 0.5, 0.6, 0.8];
                #[allow(clippy::approx_constant)]
                let expected = $vals;
                for (&input, &exp) in inputs.iter().zip(expected.iter()) {
                    assert_relative_eq!(input.$func($param), exp, epsilon = 1e-6);
                }
            }
        };
    }

    generate_reference_tests!(
        ease_in_quad,
        [0.040000, 0.160000, 0.250000, 0.360000, 0.640000]
    );
    generate_reference_tests!(
        ease_out_quad,
        [0.360000, 0.640000, 0.750000, 0.840000, 0.960000]
    );
    generate_reference_tests!(
        ease_in_out_quad,
        [0.080000, 0.320000, 0.500000, 0.680000, 0.920000]
    );
    generate_reference_tests!(
        ease_in_cubic,
        [0.008000, 0.064000, 0.125000, 0.216000, 0.512000]
    );
    generate_reference_tests!(
        ease_out_cubic,
        [0.488000, 0.784000, 0.875000, 0.936000, 0.992000]
    );
    generate_reference_tests!(
        ease_in_out_cubic,
        [0.032000, 0.256000, 0.500000, 0.744000, 0.968000]
    );
    generate_reference_tests!(
        ease_in_quart,
        [0.001600, 0.025600, 0.062500, 0.129600, 0.409600]
    );
    generate_reference_tests!(
        ease_out_quart,
        [0.590400, 0.870400, 0.937500, 0.974400, 0.998400]
    );
    generate_reference_tests!(
        ease_in_out_quart,
        [0.012800, 0.204800, 0.500000, 0.795200, 0.987200]
    );
    generate_reference_tests!(
        ease_in_quint,
        [0.000320, 0.010240, 0.031250, 0.077760, 0.327680]
    );
    generate_reference_tests!(
        ease_out_quint,
        [0.672320, 0.922240, 0.968750, 0.989760, 0.999680]
    );
    generate_reference_tests!(
        ease_in_out_quint,
        [0.005120, 0.163840, 0.500000, 0.836160, 0.994880]
    );
    generate_reference_tests!(
        ease_in_sine,
        [0.048943, 0.190983, 0.292893, 0.412215, 0.690983]
    );
    generate_reference_tests!(
        ease_out_sine,
        [0.309017, 0.587785, 0.707107, 0.809017, 0.951057]
    );
    generate_reference_tests!(
        ease_in_out_sine,
        [0.095491, 0.345492, 0.500000, 0.654509, 0.904509]
    );
    generate_reference_tests!(
        ease_in_circ,
        [0.020204, 0.083485, 0.133975, 0.200000, 0.400000]
    );
    generate_reference_tests!(
        ease_out_circ,
        [0.600000, 0.800000, 0.866025, 0.916515, 0.979796]
    );
    generate_reference_tests!(
        ease_in_out_circ,
        [0.041742, 0.200000, 0.500000, 0.800000, 0.958258]
    );
    generate_reference_tests!(
        ease_in_back,
        [-0.046451, -0.099352, -0.087698, -0.029028, 0.294198]
    );
    generate_reference_tests!(
        ease_out_back,
        [0.705802, 1.029027, 1.087698, 1.099352, 1.046_45]
    );
    generate_reference_tests!(
        ease_in_out_back,
        [-0.092556, 0.089926, 0.500000, 0.910074, 1.092556]
    );
    generate_reference_tests!(
        ease_in_bounce,
        [0.060000, 0.227500, 0.234375, 0.090000, 0.697500]
    );
    generate_reference_tests!(
        ease_out_bounce,
        [0.302500, 0.910000, 0.765625, 0.772500, 0.940000]
    );
    generate_reference_tests!(
        ease_in_out_bounce,
        [0.113750, 0.348750, 0.500000, 0.651250, 0.886250]
    );
    generate_reference_tests!(
        ease_in_expo,
        [0.003906, 0.015625, 0.031250, 0.062500, 0.250000]
    );
    generate_reference_tests!(
        ease_out_expo,
        [0.750000, 0.937500, 0.968750, 0.984375, 0.996094]
    );
    generate_reference_tests!(
        ease_in_out_expo,
        [0.007812, 0.125000, 0.500000, 0.875000, 0.992188]
    );
    generate_reference_tests!(
        ease_in_elastic,
        [-0.001953, 0.015625, -0.015625, -0.031250, -0.125000]
    );
    generate_reference_tests!(
        ease_out_elastic,
        [1.125, 1.031_25, 1.015625, 0.984375, 1.001953]
    );
    generate_reference_tests!(
        ease_in_out_elastic,
        [-0.003906, -0.117462, 0.500000, 1.117462, 1.003906]
    );
    generate_reference_tests!(
        ease_in_curve,
        1.0,
        [0.128851, 0.286231, 0.377541, 0.478454, 0.713236]
    );
    generate_reference_tests!(
        ease_out_curve,
        1.0,
        [0.286764, 0.521546, 0.622459, 0.713769, 0.871149]
    );
    generate_reference_tests!(
        ease_in_out_curve,
        1.0,
        [0.143115, 0.356618, 0.500000, 0.643382, 0.856885]
    );
}
