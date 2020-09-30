macro_rules! impl_vec_from {
  ($Scalar:ident $Vector:ident $dim:tt $($i:tt $k:ident $t:ty)*) => {
    // from scalar
    impl_vec_from!(s: $Scalar => $Vector, Self { $($k: s),* });

    // from slice
    impl_vec_from!(s: &[$Scalar] => $Vector, Self { $($k: s[$i]),* });

    // from/into array
    impl_vec_from!(a: [$Scalar; $dim] => $Vector, Self { $($k: a[$i]),* });
    impl_vec_from!(v: $Vector => [$Scalar; $dim], [$(v.$k),*]);
    impl_vec_from!(v: &$Vector => [$Scalar; $dim], [$(v.$k),*]);

    // from/into tuple
    impl_vec_from!(t: ($($t),*) => $Vector, Self { $($k: t.$i),* });
    impl_vec_from!(v: $Vector => ($($t),*), ($(v.$k),*));
    impl_vec_from!(v: &$Vector => ($($t),*), ($(v.$k),*));
  };
  ($from:ident: $From:ty => $Into:ty, $expr:expr) => {
    impl From<$From> for $Into {
      #[inline]
      fn from($from: $From) -> Self {
        $expr
      }
    }
  };
}
