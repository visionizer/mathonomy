pub use crate::consts;

pub(crate) mod private {
    pub trait Sealed {}
    impl Sealed for f64 {}
}

pub trait NumericalExtensions: private::Sealed {
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: f64) -> Self;
    fn sqrt(self) -> Self;
}

impl NumericalExtensions for f64 {
    #[inline]
    fn powi(self, n: i32) -> Self {
        unsafe { core::intrinsics::powif64(self, n) }
    }

    #[inline]
    fn powf(self, n: f64) -> Self {
        unsafe { core::intrinsics::powf64(self, n) }
    }
    #[inline]
    fn sqrt(self) -> Self {
        unsafe { core::intrinsics::sqrtf64(self) }
    }
}
