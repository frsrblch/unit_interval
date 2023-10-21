use num_traits::{One, Zero};

// TODO consider adding other ops that return Option<UnitInterval<T>>

/// A value guaranteed to be in the range `0.0..=1.0`
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnitInterval<T>(T);

impl<T: Zero + One + PartialOrd> UnitInterval<T> {
    #[inline]
    pub fn new(value: T) -> Option<Self> {
        if (T::zero()..=T::one()).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl<T> UnitInterval<T> {
    #[inline]
    pub fn get(self) -> T {
        self.0
    }
}

impl UnitInterval<f32> {
    #[inline]
    pub fn as_f64(self) -> UnitInterval<f64> {
        UnitInterval(self.0 as f64)
    }
}

impl UnitInterval<f64> {
    #[inline]
    pub fn as_f32(self) -> UnitInterval<f32> {
        UnitInterval(self.0 as f32)
    }
}

impl From<UnitInterval<f32>> for UnitInterval<f64> {
    #[inline]
    fn from(value: UnitInterval<f32>) -> Self {
        value.as_f64()
    }
}

impl From<UnitInterval<f64>> for UnitInterval<f32> {
    #[inline]
    fn from(value: UnitInterval<f64>) -> Self {
        value.as_f32()
    }
}

impl<T: PartialEq> PartialEq<T> for UnitInterval<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T: PartialOrd> PartialOrd<T> for UnitInterval<T> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul for UnitInterval<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T: std::ops::Sub<Output = T> + One> UnitInterval<T> {
    /// The complement of `x` is `1 - x`
    #[inline]
    pub fn complement(self) -> Self {
        Self(T::one() - self.0)
    }
}

impl<T: std::ops::Sub<Output = T> + One> std::ops::Not for UnitInterval<T> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        self.complement()
    }
}

macro_rules! impl_traits {
    ($ty:ty) => {
        impl PartialEq<UnitInterval<$ty>> for $ty {
            #[inline]
            fn eq(&self, other: &UnitInterval<$ty>) -> bool {
                self.eq(&other.get())
            }
        }
        impl PartialOrd<UnitInterval<$ty>> for $ty {
            #[inline]
            fn partial_cmp(&self, other: &UnitInterval<$ty>) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&other.get())
            }
        }

        impl From<UnitInterval<$ty>> for $ty {
            #[inline]
            fn from(value: UnitInterval<$ty>) -> Self {
                value.0
            }
        }

        impl rand::distributions::Distribution<UnitInterval<$ty>> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> UnitInterval<$ty> {
                UnitInterval(rng.gen_range(0.0..=1.0))
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_traits!($ty);
        )*
    };
}

impl_traits!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_tests() {
        assert_eq!(None, UnitInterval::new(-0.1));
        assert_eq!(Some(UnitInterval(0.0)), UnitInterval::new(0.0));
        assert_eq!(Some(UnitInterval(0.5)), UnitInterval::new(0.5));
        assert_eq!(Some(UnitInterval(1.0)), UnitInterval::new(1.0));
        assert_eq!(None, UnitInterval::new(1.1));
    }

    #[test]
    fn complement() {
        assert_eq!(UnitInterval(1.0), !UnitInterval(0.0));
        assert_eq!(UnitInterval(1.0 - 0.9), !UnitInterval(0.9));
        assert_eq!(UnitInterval(1.0 - 0.1), !UnitInterval(0.1));
        assert_eq!(UnitInterval(0.0), !UnitInterval(1.0));
    }

    #[test]
    fn rand_distribution() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x = rng.gen::<UnitInterval<f32>>().get();
            assert!(UnitInterval::new(x).is_some());
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn rand_generates_1() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen::<UnitInterval<f32>>();
            if x == 1.0 {
                return;
            }
        }
    }
}
