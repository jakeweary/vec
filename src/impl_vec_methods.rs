macro_rules! impl_vec_methods {
  ($Scalar:ident $Vector:ident $($k:ident)*) => {
    impl $Vector {
      $(#[inline] pub fn $k() -> Self {
        Self { $k: 1.0, ..0.0.into() }
      })*

      #[inline]
      pub fn new($($k: $Scalar),*) -> Self {
        Self { $($k),* }
      }

      #[inline]
      pub fn norm(&self) -> Self {
        self.mag().recip() * self
      }

      #[inline]
      pub fn norm_to(&self, t: $Scalar) -> Self {
        t / self.mag() * self
      }

      #[inline]
      pub fn mag(&self) -> $Scalar {
        self.mag2().sqrt()
      }

      #[inline]
      pub fn mag2(&self) -> $Scalar {
        self.dot(self)
      }

      #[inline]
      pub fn sum(&self) -> $Scalar {
        fold!(+ $(self.$k),*)
      }

      #[inline]
      pub fn product(&self) -> $Scalar {
        fold!(* $(self.$k),*)
      }

      #[inline]
      pub fn dot(&self, other: &Self) -> $Scalar {
        fold!(+ $(self.$k * other.$k),*)
      }

      #[inline]
      pub fn lerp(&self, other: &Self, t: $Scalar) -> Self {
        Self { $($k: self.$k + t * (other.$k - self.$k)),* }
      }

      #[inline]
      pub fn reflect(&self, normal: &Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
      }

      #[inline]
      pub fn refract(&self, normal: &Self, eta: $Scalar) -> Self {
        let n = self.dot(normal);
        match 1.0 - eta * eta * (1.0 - n * n) {
          k if k < 0.0 => 0.0.into(),
          k => eta * self - (eta * n + k.sqrt()) * normal
        }
      }

      #[inline]
      pub fn map<F>(&self, f: F) -> Self
      where F: Fn($Scalar) -> $Scalar {
        Self { $($k: f(self.$k)),* }
      }

      #[inline]
      pub fn zip<F>(&self, other: &Self, f: F) -> Self
      where F: Fn($Scalar, $Scalar) -> $Scalar {
        Self { $($k: f(self.$k, other.$k)),* }
      }
    }

    impl_vec_unique_methods!($Scalar $Vector $($k)*);
  }
}

macro_rules! impl_vec_unique_methods {
  ($Scalar:ident $Vector:ident $x:ident $y:ident $z:ident) => {
    impl $Vector {
      #[inline]
      pub fn cross(&self, other: &Self) -> Self {
        let $x = self.$y * other.$z - self.$z * other.$y;
        let $y = self.$z * other.$x - self.$x * other.$z;
        let $z = self.$x * other.$y - self.$y * other.$x;
        Self { $x, $y, $z }
      }
    }
  };
  ($Scalar:ident $Vector:ident $x:ident $y:ident) => {
    impl $Vector {
      #[inline]
      pub fn cross(&self, other: &Self) -> $Scalar {
        self.$x * other.$y - self.$y * other.$x
      }

      #[inline]
      pub fn rotate(&self, angle: $Scalar) -> Self {
        let (sin, cos) = angle.sin_cos();
        let $x = cos * self.$x - sin * self.$y;
        let $y = sin * self.$x + cos * self.$y;
        Self { $x, $y }
      }

      #[inline]
      pub fn left(&self) -> Self {
        Self { $x: -self.$y, $y: self.$x }
      }

      #[inline]
      pub fn right(&self) -> Self {
        Self { $x: self.$y, $y: -self.$x }
      }
    }
  };
  ($($k:tt)*) => {};
}
