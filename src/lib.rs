#![feature(clamp)]

use core::ops::*;

#[macro_use] mod impl_mat_from;
#[macro_use] mod impl_mat_methods;
#[macro_use] mod impl_mat_ops;
#[macro_use] mod impl_mat;
#[macro_use] mod impl_vec_from;
#[macro_use] mod impl_vec_inner_methods;
#[macro_use] mod impl_vec_methods;
#[macro_use] mod impl_vec_ops;
#[macro_use] mod impl_vec;
#[macro_use] mod util;

macro_rules! impl_all(() => {
  pub type Scalar = f64;

  impl_vec!(Scalar Vec2 vec2 2 x:0 y:1);
  impl_vec!(Scalar Vec3 vec3 3 x:0 y:1 z:2);
  impl_vec!(Scalar Vec4 vec4 4 x:0 y:1 z:2 w:3);

  impl_mat!(Scalar None Vec2 Mat2 mat2 2 m11 m12 m21 m22);
  impl_mat!(Scalar Vec2 Vec3 Mat3 mat3 3 m11 m12 m13 m21 m22 m23 m31 m32 m33);
  impl_mat!(Scalar Vec3 Vec4 Mat4 mat4 4 m11 m12 m13 m14 m21 m22 m23 m24 m31 m32 m33 m34 m41 m42 m43 m44);
});

impl_all!();
