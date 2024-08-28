// use wasm_bindgen::prelude::*;

mod openmaths {
  pub struct Vector2D {
    pub x: f64,
    pub y: f64,
  }

  pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
  }

  impl Vector3D {
    pub fn set(x: f64, y: f64, z: f64) -> Vector3D {
      Vector3D { x: x, y: y, z: z }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
      Vector3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
  }
}
