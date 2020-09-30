macro_rules! impl_mat_methods {
  ($Scalar:ident $SubVec:ident $Vector:ident $Matrix:ident $dim:tt $($k:ident)*) => {
    impl $Matrix {
      #[inline]
      pub fn new($($k: $Scalar),*) -> Self {
        Self { $($k),* }
      }
    }

    impl_mat_unique_methods!($Scalar $SubVec $Vector $Matrix $dim);
  }
}

macro_rules! impl_mat_unique_methods {
  ($Scalar:ident $SubVec:ident $Vector:ident $Matrix:ident 4) => {
    impl $Matrix {
      #[inline]
      pub fn rotation(axis: &$SubVec, angle: $Scalar) -> Self {
        let (x, y, z) = axis.into();
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        let (sx, sy, sz) = (s * x, s * y, s * z);
        let (tx, ty, tz) = (t * x, t * y, t * z);
        let (xx, xy, xz) = (x * tx + c, y * tx + sz, z * tx - sy);
        let (yx, yy, yz) = (x * ty - sz, y * ty + c, z * ty + sx);
        let (zx, zy, zz) = (x * tz + sy, y * tz - sx, z * tz + c);
        (xx, xy, xz, 0.0, yx, yy, yz, 0.0, zx, zy, zz, 0.0, 0.0, 0.0, 0.0, 1.0).into()
      }

      #[inline]
      pub fn translation(v: &$SubVec) -> Self {
        let (x, y, z) = v.into();
        (1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0).into()
      }

      #[inline]
      pub fn scaling(v: &$SubVec) -> Self {
        let (x, y, z) = v.into();
        (x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0).into()
      }
    }
  };
  ($Scalar:ident $SubVec:ident $Vector:ident $Matrix:ident 3) => {
    impl $Matrix {
      #[inline]
      pub fn rotation(angle: $Scalar) -> Self {
        let (s, c) = angle.sin_cos();
        (c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0).into()
      }

      #[inline]
      pub fn translation(v: &$SubVec) -> Self {
        let (x, y) = v.into();
        (1.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, 1.0).into()
      }

      #[inline]
      pub fn scaling(v: &$SubVec) -> Self {
        let (x, y) = v.into();
        (x, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 1.0).into()
      }
    }
  };
  ($Scalar:ident $SubVec:ident $Vector:ident $Matrix:ident 2) => {
    impl $Matrix {
      #[inline]
      pub fn rotation(angle: $Scalar) -> Self {
        let (s, c) = angle.sin_cos();
        (c, s, -s, c).into()
      }

      #[inline]
      pub fn scaling(v: &$Vector) -> Self {
        let (x, y) = v.into();
        (x, 0.0, 0.0, y).into()
      }
    }
  };
  ($($k:tt)*) => {};
}
