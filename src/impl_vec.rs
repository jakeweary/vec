macro_rules! impl_vec {
  ($Scalar:ident $Vector:ident $vector:ident $dim:tt $($k:ident:$i:tt)*) => {
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct $Vector {
      $(pub $k: $Scalar),*
    }

    #[inline]
    pub fn $vector($($k: $Scalar),*) -> $Vector {
      $Vector { $($k),* }
    }

    impl_vec_from!($Scalar $Vector $dim $($i $k $Scalar)*);
    impl_vec_ops!($Scalar $Vector $($k)*);
    impl_vec_methods!($Scalar $Vector $($k)*);
    impl_vec_inner_methods!($Scalar $Vector $($k)*);
  }
}
