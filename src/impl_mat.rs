macro_rules! impl_mat {
  ($Scalar:ident $SubVec:ident $Vector:ident $Matrix:ident $matrix:ident $dim:tt $($k:ident)*) => {
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct $Matrix {
      $(pub $k: $Scalar),*
    }

    #[inline]
    pub fn $matrix($($k: $Scalar),*) -> $Matrix {
      $Matrix { $($k),* }
    }

    impl_mat_from!($Scalar $Matrix $dim $($k $Scalar)*);
    impl_mat_ops!($Scalar $Vector $Matrix $dim);
    impl_mat_methods!($Scalar $SubVec $Vector $Matrix $dim $($k)*);
  }
}
