use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Matrix3 {
  elements: Vec<f64>,
}

/**
 * Matrix3 Module
 * Representation of a 3x3 Matrix
 * It's a column-major matrix.
 * When setting the elements, they are set in row-major order.
 */
#[wasm_bindgen]
impl Matrix3 {
  // #[wasm_bindgen(getter)]
  // pub fn elements(&self) -> Vec<f64> {
  //   self.elements.clone()
  // }

  // #[wasm_bindgen(setter)]
  // pub fn set_elements(&mut self, elements: Vec<f64>) {
  //   self.elements = elements;
  // }

  /**
  * Create a new Matrix3 with identity elements.
  */
  #[wasm_bindgen(constructor)]
  pub fn new() -> Matrix3 {
    Matrix3 {
      elements: vec![
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
      ],
    }
  }

  pub fn set(
    &mut self,
    m0: f64, m1: f64, m2: f64,
    m3: f64, m4: f64, m5: f64,
    m6: f64, m7: f64, m8: f64,
  ) -> Matrix3 {
    let mut column_major_elements: Vec<f64> = vec![0.0; 9];

    column_major_elements[0] = m0; column_major_elements[3] = m1; column_major_elements[6] = m2;
    column_major_elements[1] = m3; column_major_elements[4] = m4; column_major_elements[7] = m5;
    column_major_elements[2] = m6; column_major_elements[5] = m7; column_major_elements[8] = m8;

    Matrix3 {
      elements: column_major_elements,
    }
  }

  /**
  * Clone the Matrix3 instance and return a new one.
  */
  pub fn clone(&self) -> Matrix3 {
    Matrix3 {
      elements: self.elements.clone(),
    }
  }

  /**
  * Sets the matrix element to create an identity matrix.
  */
  pub fn identity(&mut self) {
    self.elements = vec![
      1.0, 0.0, 0.0,
      0.0, 1.0, 0.0,
      0.0, 0.0, 1.0,
    ];
  }

  /**
  * Determinant for a 3x3 matrix.
  * https://en.wikipedia.org/wiki/Determinant
  */
  // TODO: Buggy, need to fix
  pub fn determinant(&self) -> f64 {
    self.elements[0] * (self.elements[4] * self.elements[8] - self.elements[5] * self.elements[7])
      - self.elements[1] * (self.elements[3] * self.elements[8] - self.elements[5] * self.elements[6])
      + self.elements[2] * (self.elements[3] * self.elements[7] - self.elements[4] * self.elements[6])
  }

  pub fn adjucate(&self) -> Matrix3 {
    let mut adj = Matrix3::new();
    adj.elements[0] = self.elements[4] * self.elements[8] - self.elements[5] * self.elements[7];
    adj.elements[1] = self.elements[2] * self.elements[7] - self.elements[1] * self.elements[8];
    adj.elements[2] = self.elements[1] * self.elements[5] - self.elements[2] * self.elements[4];
    adj.elements[3] = self.elements[5] * self.elements[6] - self.elements[3] * self.elements[8];
    adj.elements[4] = self.elements[0] * self.elements[8] - self.elements[2] * self.elements[6];
    adj.elements[5] = self.elements[2] * self.elements[3] - self.elements[0] * self.elements[5];
    adj.elements[6] = self.elements[3] * self.elements[7] - self.elements[4] * self.elements[6];
    adj.elements[7] = self.elements[1] * self.elements[6] - self.elements[0] * self.elements[7];
    adj.elements[8] = self.elements[0] * self.elements[4] - self.elements[1] * self.elements[3];
    adj
  }

  pub fn inverse(&self) -> Option<Matrix3> {
    let det = self.determinant();
    if det == 0.0 {
      return None; // No inverse exists
    }
    let adj = self.adjucate();
    let mut inv = Matrix3::new();
    for i in 0..9 {
      inv.elements[i] = adj.elements[i] / det;
    }
    Some(inv)
  }

  pub fn transpose(&self) -> Matrix3 {
    let mut transposed = Matrix3::new();
    transposed.elements[0] = self.elements[0];
    transposed.elements[1] = self.elements[3];
    transposed.elements[2] = self.elements[6];
    transposed.elements[3] = self.elements[1];
    transposed.elements[4] = self.elements[4];
    transposed.elements[5] = self.elements[7];
    transposed.elements[6] = self.elements[2];
    transposed.elements[7] = self.elements[5];
    transposed.elements[8] = self.elements[8];
    transposed
  }
}
  