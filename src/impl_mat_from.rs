macro_rules! impl_mat_from {
  ($Scalar:ident $Matrix:ident $dim:tt $($k:ident $t:ty)*) => {
    // from scalar
    impl_mat_from!(s: $Scalar => $Matrix, Self { $($k: s),* });

    // from/into array
    impl_mat_from!([$($k),*]: [$Scalar; $dim*$dim] => $Matrix, Self { $($k),* });
    impl_mat_from!(m: $Matrix => [$Scalar; $dim*$dim], [$(m.$k),*]);
    impl_mat_from!(m: &$Matrix => [$Scalar; $dim*$dim], [$(m.$k),*]);

    // from/into tuple
    impl_mat_from!(($($k),*): ($($t),*) => $Matrix, Self { $($k),* });
    impl_mat_from!(m: $Matrix => ($($t),*), ($(m.$k),*));
    impl_mat_from!(m: &$Matrix => ($($t),*), ($(m.$k),*));
  };
  ($from:tt: $From:ty => $Into:ty, $expr:expr) => {
    impl From<$From> for $Into {
      #[inline]
      fn from($from: $From) -> Self {
        $expr
      }
    }
  };
}
