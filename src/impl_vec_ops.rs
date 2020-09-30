macro_rules! impl_vec_ops {
  ($Scalar:ident $Vector:ident $($k:ident)*) => {
    impl_vec_unop!($Scalar $Vector $($k)*, Neg neg -);

    impl_vec_binop!($Scalar $Vector $($k)*, Add add +);
    impl_vec_binop!($Scalar $Vector $($k)*, Sub sub -);
    impl_vec_binop!($Scalar $Vector $($k)*, Mul mul *);
    impl_vec_binop!($Scalar $Vector $($k)*, Div div /);
    impl_vec_binop!($Scalar $Vector $($k)*, Rem rem %);

    impl_vec_binop_assign!($Scalar $Vector $($k)*, AddAssign add_assign +=);
    impl_vec_binop_assign!($Scalar $Vector $($k)*, SubAssign sub_assign -=);
    impl_vec_binop_assign!($Scalar $Vector $($k)*, MulAssign mul_assign *=);
    impl_vec_binop_assign!($Scalar $Vector $($k)*, DivAssign div_assign /=);
    impl_vec_binop_assign!($Scalar $Vector $($k)*, RemAssign rem_assign %=);
  };
}

macro_rules! impl_vec_unop {
  ($Scalar:ident $Vector:ident $($k:ident)*, $Trait:ident $method:ident $op:tt) => {
    // × vector
    impl_vec_unop!($Trait $method, v:  $Vector, $Vector, $($k: $op v.$k),*);
    impl_vec_unop!($Trait $method, v: &$Vector, $Vector, $($k: $op v.$k),*);
  };
  ($Trait:ident $method:ident, $rhs:ident: $Rhs:ty, $Out:ident, $($tt:tt)*) => {
    impl $Trait for $Rhs {
      type Output = $Out;

      #[inline]
      fn $method(self) -> Self::Output {
        let $rhs = self;
        Self::Output { $($tt)* }
      }
    }
  };
}

macro_rules! impl_vec_binop {
  ($Scalar:ident $Vector:ident $($k:ident)*, $Trait:ident $method:ident $op:tt) => {
    // vector × vector
    impl_vec_binop!($Trait $method, a:  $Vector, b:  $Vector, $Vector, $($k: a.$k $op b.$k),*);
    impl_vec_binop!($Trait $method, a:  $Vector, b: &$Vector, $Vector, $($k: a.$k $op b.$k),*);
    impl_vec_binop!($Trait $method, a: &$Vector, b:  $Vector, $Vector, $($k: a.$k $op b.$k),*);
    impl_vec_binop!($Trait $method, a: &$Vector, b: &$Vector, $Vector, $($k: a.$k $op b.$k),*);

    // vector × scalar
    impl_vec_binop!($Trait $method, v:  $Vector, s: $Scalar, $Vector, $($k: v.$k $op s),*);
    impl_vec_binop!($Trait $method, v: &$Vector, s: $Scalar, $Vector, $($k: v.$k $op s),*);

    // scalar × vector
    impl_vec_binop!($Trait $method, s: $Scalar, v:  $Vector, $Vector, $($k: s $op v.$k),*);
    impl_vec_binop!($Trait $method, s: $Scalar, v: &$Vector, $Vector, $($k: s $op v.$k),*);
  };
  ($Trait:ident $method:ident, $lhs:ident: $Lhs:ty, $rhs:ident: $Rhs:ty, $Out:ident, $($tt:tt)*) => {
    impl $Trait<$Rhs> for $Lhs {
      type Output = $Out;

      #[inline]
      fn $method(self, other: $Rhs) -> Self::Output {
        let ($lhs, $rhs) = (self, other);
        Self::Output { $($tt)* }
      }
    }
  };
}

macro_rules! impl_vec_binop_assign {
  ($Scalar:ident $Vector:ident $($k:ident)*, $Trait:ident $method:ident $op:tt) => {
    // vector ×= vector
    impl_vec_binop_assign!($Trait $method, a: $Vector, b:  $Vector, $(a.$k $op b.$k;)*);
    impl_vec_binop_assign!($Trait $method, a: $Vector, b: &$Vector, $(a.$k $op b.$k;)*);

    // vector ×= scalar
    impl_vec_binop_assign!($Trait $method, v: $Vector, s: $Scalar, $(v.$k $op s;)*);
  };
  ($Trait:ident $method:ident, $lhs:ident: $Lhs:ty, $rhs:ident: $Rhs:ty, $($tt:tt)*) => {
    impl $Trait<$Rhs> for $Lhs {
      #[inline]
      fn $method(&mut self, other: $Rhs) {
        let ($lhs, $rhs) = (self, other);
        $($tt)*
      }
    }
  };
}
