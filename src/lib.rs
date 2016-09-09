#![no_std]


use core::{usize, u8, u16, u32, u64};
use core::{isize, i8, i16, i32, i64};
use core::{f32, f64};



pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! trait_bounded {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline(always)]
            fn min_value() -> $t {$min}
                #[inline(always)]
            fn max_value() -> $t {$max}
        }
    }
}

trait_bounded!(usize, usize::MIN, usize::MAX);
trait_bounded!(u8, u8::MIN, u8::MAX);
trait_bounded!(u16, u16::MIN, u16::MAX);
trait_bounded!(u32, u32::MIN, u32::MAX);
trait_bounded!(u64, u64::MIN, u64::MAX);

trait_bounded!(isize, isize::MIN, isize::MAX);
trait_bounded!(i8, i8::MIN, i8::MAX);
trait_bounded!(i16, i16::MIN, i16::MAX);
trait_bounded!(i32, i32::MIN, i32::MAX);
trait_bounded!(i64, i64::MIN, i64::MAX);

trait_bounded!(f32, f32::MIN, f32::MAX);
trait_bounded!(f64, f64::MIN, f64::MAX);


#[cfg(test)]
mod test {
    use core::{usize, isize, f32, f64};
    use super::Bounded;

    fn min_value<T: Bounded>() -> T {
        T::min_value()
    }
    fn max_value<T: Bounded>() -> T {
        T::max_value()
    }

    #[test]
    fn test() {
        assert_eq!(min_value::<usize>(), usize::MIN);
        assert_eq!(min_value::<f32>(), f32::MIN);
        assert_eq!(max_value::<f64>(), f64::MAX);
        assert_eq!(max_value::<isize>(), isize::MAX);
    }
}
