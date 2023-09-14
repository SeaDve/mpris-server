use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};
use zbus::zvariant::{Type, Value};

/// A time which can be negative.
#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type,
)]
#[serde(transparent)]
#[doc(alias = "Time_In_Us")]
pub struct Time(i64);

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}µs", self.0)
    }
}

impl Time {
    /// A time of zero.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// let time = Time::ZERO;
    /// assert!(time.is_zero());
    /// assert_eq!(time.as_micros(), 0);
    /// ```
    pub const ZERO: Self = Self::from_micros(0);

    /// The minimum time.
    pub const MIN: Self = Self::from_micros(i64::MIN);

    /// The maximum time.
    pub const MAX: Self = Self::from_micros(i64::MAX);

    /// Creates a new `Time` from the specified number of whole seconds.
    ///
    /// # Examples
    ///  ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_secs(5).as_nanos(), 5_000_000_000);
    /// ```
    #[inline]
    pub const fn from_secs(secs: i64) -> Self {
        Self::from_micros(secs * 1_000_000)
    }

    /// Creates a new `Time` from the specified number of whole milliseconds.
    ///
    /// # Examples
    ///  ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_millis(5).as_nanos(), 5_000_000);
    /// ```
    #[inline]
    pub const fn from_millis(millis: i64) -> Self {
        Self::from_micros(millis * 1000)
    }

    /// Creates a new `Time` from the specified number of whole microseconds.
    ///
    /// # Examples
    ///  ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_micros(5).as_nanos(), 5_000);
    /// ```
    #[inline]
    pub const fn from_micros(micros: i64) -> Self {
        Self(micros)
    }

    /// Creates a new `Time` from the specified number of whole nanoseconds.
    ///
    /// Note: This will round of the nanoseconds to microseconds level of
    /// precision.
    ///
    /// # Examples
    ///  ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_nanos(5).as_nanos(), 0);
    /// assert_eq!(Time::from_nanos(5342).as_nanos(), 5000);
    /// ```
    #[inline]
    pub const fn from_nanos(nanos: i64) -> Self {
        Self::from_micros(nanos / 1000)
    }

    /// Returns the number of *whole* seconds contained by this `Time`.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_micros(5_000_000).as_secs(), 5);
    /// assert_eq!(Time::from_micros(3).as_secs(), 0);
    /// ```
    #[inline]
    pub const fn as_secs(&self) -> i64 {
        self.as_micros() / 1_000_000
    }

    /// Returns the number of *whole* milliseconds contained by this `Time`.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_micros(5_000_000).as_millis(), 5_000);
    /// assert_eq!(Time::from_micros(3).as_millis(), 0);
    /// ```
    #[inline]
    pub const fn as_millis(&self) -> i64 {
        self.as_micros() / 1000
    }

    /// Returns the number of *whole* microseconds contained by this `Time`.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_micros(5_000_000).as_micros(), 5_000_000);
    /// assert_eq!(Time::from_micros(3).as_micros(), 3);
    /// ```
    #[inline]
    pub const fn as_micros(&self) -> i64 {
        self.0
    }

    /// Returns the number of *whole* nanoseconds contained by this `Time`.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::from_micros(5_000_000).as_nanos(), 5_000_000_000);
    /// assert_eq!(Time::from_micros(3).as_nanos(), 3_000);
    #[inline]
    pub const fn as_nanos(&self) -> i64 {
        self.as_micros() * 1000
    }

    /// Returns true if this `Time` is zero.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::ZERO.is_zero(), true);
    /// assert_eq!(Time::from_micros(1).is_zero(), false);
    /// ```
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if this `Time` is negative.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::ZERO.is_negative(), false);
    /// assert_eq!(Time::from_micros(-1).is_negative(), true);
    /// assert_eq!(Time::from_micros(1).is_negative(), false);
    /// ```
    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Returns true if this `Time` is positive.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::ZERO.is_positive(), false);
    /// assert_eq!(Time::from_micros(1).is_positive(), true);
    /// assert_eq!(Time::from_micros(-1).is_positive(), false);
    /// ```
    #[inline]
    pub const fn is_positive(&self) -> bool {
        self.0 > 0
    }

    /// Returns the time as an absolute (non-negative) value.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(Time::ZERO.abs(), Time::ZERO);
    /// assert_eq!(Time::from_micros(-1).abs(), Time::from_micros(1));
    /// assert_eq!(Time::from_micros(1).abs(), Time::from_micros(1));
    /// ```
    #[inline]
    pub const fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Checked `Time` addition. Computes `self + other`, returning [`None`]
    /// if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(
    ///     Time::from_micros(1).checked_add(Time::from_micros(1)),
    ///     Some(Time::from_micros(2))
    /// );
    /// assert_eq!(Time::MAX.checked_add(Time::from_micros(1)), None);
    /// ```
    #[inline]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(inner) => Some(Self(inner)),
            None => None,
        }
    }

    /// Checked `Time` subtraction. Computes `self - other`, returning [`None`]
    /// if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(
    ///     Time::from_micros(2).checked_sub(Time::from_micros(1)),
    ///     Some(Time::from_micros(1))
    /// );
    /// assert_eq!(Time::MIN.checked_sub(Time::from_micros(1)), None);
    /// ```
    #[inline]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(inner) => Some(Self(inner)),
            None => None,
        }
    }

    /// Saturating `Time` addition. Computes `self + other`, returning
    /// [`Time::MAX`] if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(
    ///     Time::from_micros(1).saturating_add(Time::from_micros(1)),
    ///     Time::from_micros(2)
    /// );
    /// assert_eq!(Time::MAX.saturating_add(Time::from_micros(1)), Time::MAX);
    /// ```
    #[inline]
    pub const fn saturating_add(self, other: Self) -> Self {
        match self.checked_add(other) {
            Some(inner) => inner,
            None => Self::MAX,
        }
    }

    /// Saturating `Time` subtraction. Computes `self - other`, returning
    /// [`Time::MIN`] if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use mpris_server::Time;
    ///
    /// assert_eq!(
    ///     Time::from_micros(2).saturating_sub(Time::from_micros(1)),
    ///     Time::from_micros(1)
    /// );
    /// assert_eq!(Time::MIN.saturating_sub(Time::from_micros(1)), Time::MIN);
    /// ```
    #[inline]
    pub const fn saturating_sub(self, other: Self) -> Self {
        match self.checked_sub(other) {
            Some(inner) => inner,
            None => Self::MIN,
        }
    }
}

impl Add for Time {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        self.checked_add(other).expect("overflow when adding times")
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Time {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.checked_sub(other)
            .expect("overflow when subtracting times")
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<'a> From<Time> for Value<'a> {
    fn from(time: Time) -> Self {
        Value::new(time.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_signature() {
        assert_eq!(Time::signature(), "x");
    }
}
