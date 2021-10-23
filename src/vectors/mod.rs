pub mod vec2;
pub mod vec3;

pub trait GenericVector: crate::prelude::private::Sealed {
    fn cross(self, other: Self) -> Self;

    fn dot(self, other: Self) -> Self;

    fn vadd(self, other: Self) -> Self;

    fn add_f64(self, by: f64) -> Self;

    fn vsub(self, other: Self) -> Self;

    fn sub_f64(self, by: f64) -> Self;

    fn vdiv(self, other: Self) -> Self;

    fn div_f64(self, by: f64) -> Self;

    fn vmul(self, other: Self) -> Self;

    fn mul_f64(self, by: f64) -> Self;

    fn all_eq(self) -> bool;
}

pub(crate) mod vec_macros {
    #[macro_export]
    macro_rules! impl_common_ops_for_vec {
        ($n:ident) => {
            impl core::ops::BitXor<$n> for $n {
                type Output = Self;
                fn bitxor(self, rhs: $n) -> Self {
                    self.cross(rhs)
                }
            }

            impl core::ops::BitOr<$n> for $n {
                type Output = Self;
                fn bitor(self, rhs: $n) -> Self {
                    self.dot(rhs)
                }
            }

            impl core::ops::Add<$n> for $n {
                type Output = Self;
                fn add(self, rhs: $n) -> Self {
                    self.vadd(rhs)
                }
            }

            impl core::ops::Add<f64> for $n {
                type Output = Self;
                fn add(self, rhs: f64) -> Self {
                    self.add_f64(rhs)
                }
            }

            impl core::ops::Sub<$n> for $n {
                type Output = Self;
                fn sub(self, rhs: $n) -> Self {
                    self.vsub(rhs)
                }
            }

            impl core::ops::Sub<f64> for $n {
                type Output = Self;
                fn sub(self, rhs: f64) -> Self {
                    self.sub_f64(rhs)
                }
            }

            impl core::ops::Mul<$n> for $n {
                type Output = Self;
                fn mul(self, rhs: $n) -> Self {
                    self.vmul(rhs)
                }
            }

            impl core::ops::Mul<f64> for $n {
                type Output = Self;
                fn mul(self, rhs: f64) -> Self {
                    self.mul_f64(rhs)
                }
            }

            impl core::ops::Div<$n> for $n {
                type Output = Self;
                fn div(self, rhs: $n) -> Self {
                    self.vdiv(rhs)
                }
            }

            impl core::ops::Div<f64> for $n {
                type Output = Self;
                fn div(self, rhs: f64) -> Self {
                    self.div_f64(rhs)
                }
            }

            impl crate::prelude::private::Sealed for $n {}
        };
    }
}
