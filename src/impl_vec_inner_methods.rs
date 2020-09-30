macro_rules! impl_vec_inner_methods {
  ($Scalar:ident $Vector:ident $($k:ident)*) => {
    impl_vec_inner_method!($Scalar $Vector $($k)*, abs);
    impl_vec_inner_method!($Scalar $Vector $($k)*, acos);
    impl_vec_inner_method!($Scalar $Vector $($k)*, acosh);
    impl_vec_inner_method!($Scalar $Vector $($k)*, asin);
    impl_vec_inner_method!($Scalar $Vector $($k)*, asinh);
    impl_vec_inner_method!($Scalar $Vector $($k)*, atan);
    impl_vec_inner_method!($Scalar $Vector $($k)*, atanh);
    impl_vec_inner_method!($Scalar $Vector $($k)*, cbrt);
    impl_vec_inner_method!($Scalar $Vector $($k)*, ceil);
    impl_vec_inner_method!($Scalar $Vector $($k)*, cos);
    impl_vec_inner_method!($Scalar $Vector $($k)*, cosh);
    impl_vec_inner_method!($Scalar $Vector $($k)*, exp);
    impl_vec_inner_method!($Scalar $Vector $($k)*, exp2);
    impl_vec_inner_method!($Scalar $Vector $($k)*, exp_m1);
    impl_vec_inner_method!($Scalar $Vector $($k)*, floor);
    impl_vec_inner_method!($Scalar $Vector $($k)*, fract);
    impl_vec_inner_method!($Scalar $Vector $($k)*, ln);
    impl_vec_inner_method!($Scalar $Vector $($k)*, ln_1p);
    impl_vec_inner_method!($Scalar $Vector $($k)*, log10);
    impl_vec_inner_method!($Scalar $Vector $($k)*, log2);
    impl_vec_inner_method!($Scalar $Vector $($k)*, recip);
    impl_vec_inner_method!($Scalar $Vector $($k)*, round);
    impl_vec_inner_method!($Scalar $Vector $($k)*, signum);
    impl_vec_inner_method!($Scalar $Vector $($k)*, sin);
    impl_vec_inner_method!($Scalar $Vector $($k)*, sinh);
    impl_vec_inner_method!($Scalar $Vector $($k)*, sqrt);
    impl_vec_inner_method!($Scalar $Vector $($k)*, tan);
    impl_vec_inner_method!($Scalar $Vector $($k)*, tanh);
    impl_vec_inner_method!($Scalar $Vector $($k)*, to_degrees);
    impl_vec_inner_method!($Scalar $Vector $($k)*, to_radians);
    impl_vec_inner_method!($Scalar $Vector $($k)*, trunc);

    impl_vec_inner_method!($Scalar $Vector $($k)*, atan2, other);
    impl_vec_inner_method!($Scalar $Vector $($k)*, copysign, sign);
    impl_vec_inner_method!($Scalar $Vector $($k)*, div_euclid, rhs);
    impl_vec_inner_method!($Scalar $Vector $($k)*, hypot, other);
    impl_vec_inner_method!($Scalar $Vector $($k)*, log, base);
    impl_vec_inner_method!($Scalar $Vector $($k)*, max, other);
    impl_vec_inner_method!($Scalar $Vector $($k)*, min, other);
    impl_vec_inner_method!($Scalar $Vector $($k)*, powf, n);
    impl_vec_inner_method!($Scalar $Vector $($k)*, rem_euclid, rhs);

    impl_vec_inner_method!($Scalar $Vector $($k)*, clamp, min, max);
    impl_vec_inner_method!($Scalar $Vector $($k)*, mul_add, a, b);
  };
}

macro_rules! impl_vec_inner_method {
  ($Scalar:ident $Vector:ident $($k:ident)*, $method:ident) => {
    impl $Vector {
      #[inline]
      pub fn $method(&self) -> Self {
        Self { $($k: self.$k.$method()),* }
      }
    }
  };
  ($Scalar:ident $Vector:ident $($k:ident)*, $method:ident, $a:ident) => {
    impl $Vector {
      #[inline]
      pub fn $method(&self, $a: &Self) -> Self {
        Self { $($k: self.$k.$method($a.$k)),* }
      }
    }
  };
  ($Scalar:ident $Vector:ident $($k:ident)*, $method:ident, $a:ident, $b:ident) => {
    impl $Vector {
      #[inline]
      pub fn $method(&self, $a: &Self, $b: &Self) -> Self {
        Self { $($k: self.$k.$method($a.$k, $b.$k)),* }
      }
    }
  };
}
