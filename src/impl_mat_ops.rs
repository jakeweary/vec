macro_rules! impl_mat_ops {
  ($Scalar:ident $Vector:ident $Matrix:ident $dim:tt) => {
    impl Mul for $Matrix {
      type Output = Self;

      #[inline]
      fn mul(self, other: Self) -> Self::Output {
        let a: [_; $dim*$dim] = self.into();
        let b: [_; $dim*$dim] = other.into();
        let mut out = [0.0; $dim*$dim];
        for i in 0..$dim {
          for j in 0..$dim {
            for k in 0..$dim {
              out[$dim*i + j] += a[$dim*i + k] * b[$dim*k + j];
            }
          }
        }
        out.into()
      }
    }

    impl Mul<$Vector> for $Matrix {
      type Output = $Vector;

      #[inline]
      fn mul(self, other: $Vector) -> Self::Output {
        let m: [_; $dim*$dim] = self.into();
        let v: [_; $dim] = other.into();
        let mut out = [0.0; $dim];
        for i in 0..$dim {
          for j in 0..$dim {
            out[i] += m[$dim*i + j] * v[j];
          }
        }
        out.into()
      }
    }

    impl Mul<$Matrix> for $Vector {
      type Output = $Vector;

      #[inline]
      fn mul(self, other: $Matrix) -> Self::Output {
        let v: [_; $dim] = self.into();
        let m: [_; $dim*$dim] = other.into();
        let mut out = [0.0; $dim];
        for i in 0..$dim {
          for j in 0..$dim {
            out[i] += m[$dim*j + i] * v[j];
          }
        }
        out.into()
      }
    }
  }
}
