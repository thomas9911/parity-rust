pub fn add<V: Parity, W: Parity>(v: V, w: W) -> bool {
    v.is_even() == w.is_even()
}

pub fn subtract<V: Parity, W: Parity>(v: V, w: W) -> bool {
    v.is_even() == w.is_even()
}

pub fn multiply<V: Parity, W: Parity>(v: V, w: W) -> bool {
    v.is_even() | w.is_even()
}

pub fn power<V: Parity, W: Parity>(v: V, _: W) -> bool {
    v.is_even()
}

pub fn tetration<V: Parity, W: Parity>(v: V, w: W) -> bool {
    power(v, w)
}

/// get the parity of an up-arrow function
///
/// n=0 ; v w
/// n=1 ; v ↑ w
/// n=2 ; v ↑↑ w
/// n=3 ; v ↑↑↑ w
/// n=4 ; v ↑↑↑↑ w
/// n=n ; v ↑n w
pub fn arrow<V: Parity, W: Parity>(v: V, n: usize, w: W) -> bool {
    match n {
        0 => multiply(v, w),
        _ => power(v, w),
    }
}

pub trait Parity {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

macro_rules! impl_parity {
    ($integer:ty) => {
        impl Parity for $integer {
            fn is_even(&self) -> bool {
                // using & 1, produces same asm
                self % 2 == 0
            }
        }
    };
}

impl Parity for bool {
    fn is_even(&self) -> bool {
        *self
    }
}

impl_parity!(u8);
impl_parity!(u16);
impl_parity!(u32);
impl_parity!(u64);
impl_parity!(usize);
impl_parity!(u128);

impl_parity!(i8);
impl_parity!(i16);
impl_parity!(i32);
impl_parity!(i64);
impl_parity!(isize);
impl_parity!(i128);

#[cfg(test)]
mod tests {
    use crate::Parity;
    use crate::{add, multiply, power, subtract};

    #[test]
    fn it_works() {
        assert!(3u8.is_odd());
        assert!(4u8.is_even());
    }

    #[test]
    fn it_works_negative() {
        assert!((-3i8).is_odd());
        assert!((-4i8).is_even());
    }

    #[test]
    fn add_test() {
        assert_eq!(add(5, 4), (5 + 4).is_even());
        assert_eq!(add(5, 5), (5 + 5).is_even());
        assert_eq!(add(2, 2), (2 + 2).is_even());
        assert_eq!(add(9, 4), (9 + 4).is_even());
    }

    #[test]
    fn subtract_test() {
        assert_eq!(subtract(5, 4), (5 - 4).is_even());
        assert_eq!(subtract(5, 5), (5 - 5).is_even());
        assert_eq!(subtract(2, 2), (2 - 2).is_even());
        assert_eq!(subtract(9, 4), (9 - 4).is_even());
    }

    #[test]
    fn multiply_test() {
        assert_eq!(multiply(3, 5), (3 * 5).is_even());
        assert_eq!(multiply(16, 6), (16 * 6).is_even());
        assert_eq!(multiply(3, 8), (3 * 8).is_even());
        assert_eq!(multiply(12, 5), (12 * 5).is_even());
    }

    #[test]
    fn power_test() {
        assert_eq!(power(3, 5), (3u32.pow(5u32)).is_even());
        assert_eq!(power(16, 6), (16u32.pow(6u32)).is_even());
        assert_eq!(power(3, 8), (3u32.pow(8u32)).is_even());
        assert_eq!(power(12, 5), (12u32.pow(5u32)).is_even());
    }

    #[test]
    fn usage() {
        // find the parity of a large number

        // (845**123 + 12) ** (547 * 845**814)

        let result = power(add(power(854, 123), 12), multiply(power(845, 814), 547));
        assert_eq!(result, true);

        // (547 * 845**81254) ** (8541**12345 + 123)

        let result = power(
            multiply(power(845, 81254), 547),
            add(power(8541, 12345), 123),
        );
        assert_eq!(result, false);
    }

    #[test]
    fn usage_on_function() {
        // x*x + 2
        let func = |x: u64| add(multiply(x, x), 2u8);

        let n0 = 0..;

        // 0, 3, 6, 11
        let first_4: Vec<_> = n0.map(func).take(4).collect();

        assert_eq!(vec![true, false, true, false], first_4);
    }
}
