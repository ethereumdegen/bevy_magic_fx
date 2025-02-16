// taken from the interpolation crate -- need to vendor to add serde 

//! A module contains implementation of ease functions.

use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize,Default)]
pub enum EaseFunction {
	#[default]
	Linear, 


    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,

    CubicIn,
    CubicOut,
    CubicInOut,

    QuarticIn,
    QuarticOut,
    QuarticInOut,

    QuinticIn,
    QuinticOut,
    QuinticInOut,

    SineIn,
    SineOut,
    SineInOut,

    CircularIn,
    CircularOut,
    CircularInOut,

    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,

    ElasticIn,
    ElasticOut,
    ElasticInOut,

    BackIn,
    BackOut,
    BackInOut,

    BounceIn,
    BounceOut,
    BounceInOut,
}

#[allow(missing_docs)]
pub trait Ease {
    /// Calculate the eased value, normalized
    fn calc(self, f: EaseFunction) -> Self;

    fn linear(self) -> Self;

    fn quadratic_in(self) -> Self;
    fn quadratic_out(self) -> Self;
    fn quadratic_in_out(self) -> Self;

    fn cubic_in(self) -> Self;
    fn cubic_out(self) -> Self;
    fn cubic_in_out(self) -> Self;

    fn quartic_in(self) -> Self;
    fn quartic_out(self) -> Self;
    fn quartic_in_out(self) -> Self;

    fn quintic_in(self) -> Self;
    fn quintic_out(self) -> Self;
    fn quintic_in_out(self) -> Self;

    fn sine_in(self) -> Self;
    fn sine_out(self) -> Self;
    fn sine_in_out(self) -> Self;

    fn circular_in(self) -> Self;
    fn circular_out(self) -> Self;
    fn circular_in_out(self) -> Self;

    fn exponential_in(self) -> Self;
    fn exponential_out(self) -> Self;
    fn exponential_in_out(self) -> Self;

    fn elastic_in(self) -> Self;
    fn elastic_out(self) -> Self;
    fn elastic_in_out(self) -> Self;

    fn back_in(self) -> Self;
    fn back_out(self) -> Self;
    fn back_in_out(self) -> Self;

    fn bounce_in(self) -> Self;
    fn bounce_out(self) -> Self;
    fn bounce_in_out(self) -> Self;
}

macro_rules! impl_ease_trait_for {
    ($T: ident) => (
        mod $T {
            pub const PI_2: $T = 6.28318530717958647692528676655900576;

            pub fn clamp(p: $T) -> $T {
                match () {
                    _ if p > 1.0 => 1.0,
                    _ if p < 0.0 => 0.0,
                    _ => p
                }
            }
        }
        impl Ease for $T {
            fn calc(self, f: EaseFunction) -> Self {
                match f {
                    EaseFunction::Linear => self.linear(),
                    EaseFunction::QuadraticIn => self.quadratic_in(),
                    EaseFunction::QuadraticOut => self.quadratic_out(),
                    EaseFunction::QuadraticInOut => self.quadratic_in_out(),

                    EaseFunction::CubicIn => self.cubic_in(),
                    EaseFunction::CubicOut => self.cubic_out(),
                    EaseFunction::CubicInOut => self.cubic_in_out(),

                    EaseFunction::QuarticIn => self.quartic_in(),
                    EaseFunction::QuarticOut => self.quartic_out(),
                    EaseFunction::QuarticInOut => self.quartic_in_out(),

                    EaseFunction::QuinticIn => self.quintic_in(),
                    EaseFunction::QuinticOut => self.quintic_out(),
                    EaseFunction::QuinticInOut => self.quintic_in_out(),

                    EaseFunction::SineIn => self.sine_in(),
                    EaseFunction::SineOut => self.sine_out(),
                    EaseFunction::SineInOut => self.sine_in_out(),

                    EaseFunction::CircularIn => self.circular_in(),
                    EaseFunction::CircularOut => self.circular_out(),
                    EaseFunction::CircularInOut => self.circular_in_out(),

                    EaseFunction::ExponentialIn => self.exponential_in(),
                    EaseFunction::ExponentialOut => self.exponential_out(),
                    EaseFunction::ExponentialInOut => self.exponential_in_out(),

                    EaseFunction::ElasticIn => self.elastic_in(),
                    EaseFunction::ElasticOut => self.elastic_out(),
                    EaseFunction::ElasticInOut => self.elastic_in_out(),

                    EaseFunction::BackIn => self.back_in(),
                    EaseFunction::BackOut => self.back_out(),
                    EaseFunction::BackInOut => self.back_in_out(),

                    EaseFunction::BounceIn => self.bounce_in(),
                    EaseFunction::BounceOut => self.bounce_out(),
                    EaseFunction::BounceInOut => self.bounce_in_out(),
                }
            }

            fn linear(self) -> Self {
                let p = $T::clamp(self);
                p 
            }

            fn quadratic_in(self) -> Self {
                let p = $T::clamp(self);
                p * p
            }

            fn quadratic_out(self) -> Self {
                let p = $T::clamp(self);
                -(p * (p - 2.0))
            }

            fn quadratic_in_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 0.5 {
                    2.0 * p * p
                } else {
                    (-2.0 * p * p) + (4.0 * p) - 1.0
                }
            }


            fn cubic_in(self) -> Self {
                let p = $T::clamp(self);
                p * p * p
            }

            fn cubic_out(self) -> Self {
                let p = $T::clamp(self);
                let f = p - 1.0;
                f * f * f + 1.0
            }

            fn cubic_in_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 0.5 {
                    4.0 * p * p * p
                } else {
                    let f = (2.0 * p) - 2.0;
                    0.5 * f * f * f + 1.0
                }
            }


            fn quartic_in(self) -> Self {
                let p = $T::clamp(self);
                p * p * p * p
            }

            fn quartic_out(self) -> Self {
                let p = $T::clamp(self);
                let f = p - 1.0;
                f * f * f * (1.0 - p) + 1.0
            }

            fn quartic_in_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 0.5 {
                    8.0 * p * p * p * p
                } else {
                    let f = p - 1.0;
                    -8.0 * f * f * f * f + 1.0
                }
            }


            fn quintic_in(self) -> Self {
                let p = $T::clamp(self);
                p * p * p * p * p
            }

            fn quintic_out(self) -> Self {
                let p = $T::clamp(self);
                let f = p - 1.0;
                f * f * f * f * f + 1.0
            }

            fn quintic_in_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 0.5  {
                    16.0 * p * p * p * p * p
                } else {
                    let f = (2.0 * p) - 2.0;
                    0.5 * f * f * f * f * f + 1.0
                }
            }


            fn sine_in(self) -> Self {
                use self::$T::PI_2;
                let p = $T::clamp(self);
                ((p - 1.0) * PI_2).sin() + 1.0
            }

            fn sine_out(self) -> Self {
                use self::$T::PI_2;
                let p = $T::clamp(self);
                (p * PI_2).sin()
            }

            fn sine_in_out(self) -> Self {
                use std::$T::consts::PI;
                let p = $T::clamp(self);
                0.5 * (1.0 - (p * PI).cos())
            }


            fn circular_in(self) -> Self {
                let p = $T::clamp(self);
                1.0 - (1.0 - (p * p)).sqrt()
            }

            fn circular_out(self) -> Self {
                let p = $T::clamp(self);
                ((2.0 - p) * p).sqrt()
            }

            fn circular_in_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * (p * p)).sqrt())
                } else {
                    0.5 * ((-((2.0 * p) - 3.0) * ((2.0 * p) - 1.0)).sqrt() + 1.0)
                }
            }


            fn exponential_in(self) -> Self {
                if self <= 0.0 {
                    0.0
                } else {
                    (2.0 as $T).powf(10.0 * (self.min(1.0) - 1.0))
                }
            }

            fn exponential_out(self) -> Self {
                if self >= 1.0 {
                    1.0
                } else {
                    1.0 - (2.0 as $T).powf(-10.0 * self.max(0.0))
                }
            }

            fn exponential_in_out(self) -> Self {
                if self <= 0.0 {
                    return 0.0;
                }
                if self >= 1.0 {
                    return 1.0;
                }

                if self < 0.5  {
                    0.5 * (2.0 as $T).powf((20.0 * self) - 10.0)
                } else {
                    -0.5 * (2.0 as $T).powf((-20.0 * self) + 10.0) + 1.0
                }
            }


            fn elastic_in(self) -> Self {
                use self::$T::PI_2;
                let p = $T::clamp(self);
                (13.0 * PI_2 * p).sin() * (2.0 as $T).powf(10.0 * (p - 1.0))
            }

            fn elastic_out(self) -> Self {
                use self::$T::PI_2;
                let p = $T::clamp(self);
                (-13.0 * PI_2 * (p + 1.0)).sin() * (2.0 as $T).powf(-10.0 * p) + 1.0
            }

            fn elastic_in_out(self) -> Self {
                use self::$T::PI_2;
                let p = $T::clamp(self);
                if p < 0.5 {
                    0.5 * (13.0 * PI_2 * (2.0 * p)).sin() * (2.0 as $T).powf(10.0 * ((2.0 * p) - 1.0))
                } else {
                    0.5 * ((-13.0 * PI_2 * ((2.0 * p - 1.0) + 1.0)).sin()
                           * (2.0 as $T).powf(-10.0 * (2.0 * p - 1.0)) + 2.0)
                }
            }


            fn back_in(self) -> Self {
                use std::$T::consts::PI;
                let p = $T::clamp(self);
                p * p * p - p * (p * PI).sin()
            }

            fn back_out(self) -> Self {
                use std::$T::consts::PI;
                let p = $T::clamp(self);
                let f = 1.0 - p;
                1.0 - (f * f * f - f * (f * PI).sin())
            }

            fn back_in_out(self) -> Self {
                use std::$T::consts::PI;
                let p = $T::clamp(self);
                if p < 0.5 {
                    let f = 2.0 * p;
                    0.5 * (f * f * f - f * (f * PI).sin())
                } else {
                    let f = 1.0 - (2.0 * p - 1.0);
                    0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
                }
            }


            fn bounce_in(self) -> Self {
                let p = $T::clamp(self);
                1.0 - Ease::bounce_out(1.0 - p)
            }

            fn bounce_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 4.0 / 11.0 {
                    (121.0 * p * p) / 16.0
                } else if p < 8.0 / 11.0 {
                    (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
                } else if p < 9.0 / 10.0 {
                    (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
                } else {
                    (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
                }
            }

            fn bounce_in_out(self) -> Self {
                let p = $T::clamp(self);
                if p < 0.5 {
                    0.5 * Ease::bounce_in(p * 2.0)
                } else {
                    0.5 * Ease::bounce_out(p * 2.0 - 1.0) + 0.5
                }
            }
        }
    )
}

impl_ease_trait_for!(f32);
impl_ease_trait_for!(f64);


// ---- 

 

/// Performs linear interpolation.
/// A linear interpolation consists of two states 'a' and 'b'.
/// The 't' variable is a factor between 0 and 1 that
/// gives weight to 'a' or 'b'.
/// When 't' is zero then 'a' has full weight.
/// When 't' is one then 'b' has full weight.
#[inline(always)]
pub fn lerp<T: Lerp>(a: &T, b: &T, t: &T::Scalar) -> T {
    a.lerp(b, t)
}

/// Describes a type that can linearly interpolate between two points.
pub trait Lerp {
    /// The scaling type for linear interpolation.
    type Scalar;

    /// Given `self` and another point `other`, return a point on a line running between the two
    /// that is `scalar` fraction of the distance between the two points.
    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self;
}

/// Implementation of `Lerp` for floats.
macro_rules! impl_lerp_for_float {
    ($float: ident) => (
        impl Lerp for $float {
            type Scalar = $float;

            #[inline(always)]
            fn lerp(&self, other: &$float, scalar: &$float) -> $float {
                self + (other - self) * scalar
            }
        }
    )
}

impl_lerp_for_float!(f32);
impl_lerp_for_float!(f64);

/// Implementation of `Lerp` for signed integers.
/// This will cast the int to the Scalar before multiplying and rounding to the nearest value.
macro_rules! impl_lerp_for_int {
    ($int: ident, $scalar: ident) => (
        impl Lerp for $int {
            type Scalar = $scalar;

            #[inline(always)]
            fn lerp(&self, other: &$int, scalar: &$scalar) -> $int {
                self + ((other - self) as $scalar * scalar).round() as $int
            }
        }
    )
}

impl_lerp_for_int!(i8,  f32);
impl_lerp_for_int!(i16, f32);
impl_lerp_for_int!(i32, f32);
impl_lerp_for_int!(i64, f64);

/// Implementation of `Lerp` for unsigned integers.
/// Will cast the uint to the Scalar before multiplying and rounding to the nearest value.
macro_rules! impl_lerp_for_uint {
    ($uint: ident, $scalar: ident) => (
        impl Lerp for $uint {
            type Scalar = $scalar;

            #[inline(always)]
            fn lerp(&self, other: &$uint, scalar: &$scalar) -> $uint {
                if self <= other {
                    self + ((other - self) as $scalar * scalar).round() as $uint
                } else {
                    self - ((self - other) as $scalar * scalar).round() as $uint
                }
            }
        }
    )
}

impl_lerp_for_uint!(u8,  f32);
impl_lerp_for_uint!(u16, f32);
impl_lerp_for_uint!(u32, f32);
impl_lerp_for_uint!(u64, f64);

/// Transitive impl of `Lerp` for arrays, given a length and index list
macro_rules! impl_lerp_for_array {
    ($len:expr; $($i:expr),*) => {
        impl<T> Lerp for [T; $len] where T: Lerp {
            type Scalar = T::Scalar;
            
            #[inline(always)]
            fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
                [
                    $(self[$i].lerp(&other[$i], scalar)),*
                ]
            }
        }
    }
}

impl_lerp_for_array!(1; 0);
impl_lerp_for_array!(2; 0, 1);
impl_lerp_for_array!(3; 0, 1, 2);
impl_lerp_for_array!(4; 0, 1, 2, 3);
impl_lerp_for_array!(5; 0, 1, 2, 3, 4);


// ----



/// Performs quadratic beziér interpolation.
/// This is done by nesting linear interpolations.
/// For more information, see:
///
/// [Beziér Curve at Wikipedia](http://en.wikipedia.org/wiki/B%C3%A9zier_curve)
#[inline(always)]
pub fn quad_bez<T: Lerp>(
    x0: &T,
    x1: &T,
    x2: &T,
    t: &T::Scalar
) -> T {
    let x_0_1 = lerp(x0, x1, t);
    let x_1_2 = lerp(x1, x2, t);
    lerp(&x_0_1, &x_1_2, t)
}

/// Performs cubic beziér interpolation.
/// This is done by interpolation between two quadratic beziér.
/// For more information, see:
///
/// [Beziér Curve at Wikipedia](http://en.wikipedia.org/wiki/B%C3%A9zier_curve)
#[inline(always)]
pub fn cub_bez<T: Lerp>(
    x0: &T,
    x1: &T,
    x2: &T,
    x3: &T,
    t: &T::Scalar
) -> T {
    let x_0_2 = quad_bez(x0, x1, x2, t);
    let x_1_3 = quad_bez(x1, x2, x3, t);
    lerp(&x_0_2, &x_1_3, t)
}