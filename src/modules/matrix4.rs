use crate::modules::matrix3::Matrix3;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/**
 * Matrix4 Module
 * 2D Representation of a 4x4 Matrix
 * It's a row-major matrix.
 */
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Matrix4 {
  elements: Vec<f64>,
}

#[wasm_bindgen]
impl Matrix4 {
  #[wasm_bindgen(getter)]
  pub fn elements(&self) -> Vec<f64> {
    self.elements.clone()
  }

  // #[wasm_bindgen(setter)]
  // pub fn set_elements(&mut self, elements: Vec<f64>) {
  //   self.elements = elements;
  // }

  /**
  * Create a new Matrix4 with identity elements.
  */
  #[wasm_bindgen(constructor)]
  pub fn new() -> Matrix4 {
    Matrix4 {
      elements: vec![
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
      ],
    }
  }

  pub fn set(
    m0: f64, m1: f64, m2: f64, m3: f64,
    m4: f64, m5: f64, m6: f64, m7: f64,
    m8: f64, m9: f64, m10: f64, m11: f64,
    m12: f64, m13: f64, m14: f64, m15: f64,
  ) -> Matrix4 {
    let mut column_major_elements: Vec<f64> = vec![0.0; 16];

    column_major_elements[0] = m0; column_major_elements[4] = m1; column_major_elements[8] = m2; column_major_elements[12] = m3;
    column_major_elements[1] = m4; column_major_elements[5] = m5; column_major_elements[9] = m6; column_major_elements[13] = m7;
    column_major_elements[2] = m8; column_major_elements[6] = m9; column_major_elements[10] = m10; column_major_elements[14] = m11;
    column_major_elements[3] = m12; column_major_elements[7] = m13; column_major_elements[11] = m14; column_major_elements[15] = m15;

    Matrix4 {
      elements: column_major_elements,
    }
  }

  /**
  * Clone the Matrix4 instance and return a new one.
  */
  pub fn clone(&self) -> Matrix4 {
    Matrix4 {
      elements: self.elements.clone(),
    }
  }

  /**
  * Sets the matrix element to create an identity matrix.
  */
  pub fn identity(&mut self) {
    self.elements = vec![
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ];
  }

  /**
  * Determinant for a 4x4 matrix.
  * https://en.wikipedia.org/wiki/Determinant
  */
  // TODO: Buggy, need to fix because it uses a 3x3 matrix determinant
  pub fn determinant(&self) -> f64 {
    let mut det = 0.0;
    // Calculate the determinant using the first row
    // and the 3x3 minor matrix, then recursively calculate the determinant for each minor.
    for i in 0..4 {
      let mut minor = [[0.0; 3]; 3];
      for j in 1..4 {
        for k in 0..4 {
          if k < i {
            minor[j - 1][k] = self.elements[j * 4 + k];
          } else if k > i {
            minor[j - 1][k - 1] = self.elements[j * 4 + k];
          }
        }
      }
      let minor_vec: Vec<f64> = minor.iter().flat_map(|row| row.iter()).cloned().collect();
      let mut minor_matrix = Matrix3::new();
      minor_matrix.set(
        minor_vec[0], minor_vec[1], minor_vec[2],
        minor_vec[3], minor_vec[4], minor_vec[5],
        minor_vec[6], minor_vec[7], minor_vec[8],
      );
      det += if i % 2 == 0 { self.elements[i] } else { -self.elements[i] } * minor_matrix.determinant();
    }
    det
  }

  pub fn adjucate(&self) -> Matrix4 {
    let mut adj = Matrix4::new();
    for i in 0..4 {
      for j in 0..4 {
        let mut sub_matrix = [[0.0; 3]; 3];
        for m in 0..4 {
          for n in 0..4 {
            if m != i && n != j {
              let sub_i = if m < i { m } else { m - 1 };
              let sub_j = if n < j { n } else { n - 1 };
              sub_matrix[sub_i][sub_j] = self.elements[m * 4 + n];
            }
          }
        }
        let mut sub_matrix3 = Matrix3::new();
        let sub_matrix_vec: Vec<f64> = sub_matrix.iter().flat_map(|row| row.iter()).cloned().collect();
        sub_matrix3.set(
            sub_matrix_vec[0], sub_matrix_vec[1], sub_matrix_vec[2],
            sub_matrix_vec[3], sub_matrix_vec[4], sub_matrix_vec[5],
            sub_matrix_vec[6], sub_matrix_vec[7], sub_matrix_vec[8],
        );
        adj.elements[i * 4 + j] = if (i + j) % 2 == 0 { 1.0 } else { -1.0 } * sub_matrix3.determinant();
      }
    }
    adj
  }

  /**
  * Inverse of a matrix
  * Returns a new Matrix4 instance.
  * If the determinant is zero, it returns a zero matrix.
  * It can be checked with is_zero() method.
  */
  pub fn inverse(&self) -> Matrix4 {
    let det = self.determinant();
    if det == 0.0 {
      // Return 0 matrix if the determinant is zero
      return Matrix4::new();
    }
    let adj = self.adjucate();
    let mut inv = Matrix4::new();
    for i in 0..16 {
      inv.elements[i] = adj.elements[i] / det;
    }
    inv
  }

  /**
  * Multiply this matrix with another Matrix4
  * Returns a new Matrix4 instance
  */
  pub fn multiply(&self, other: &Matrix4) -> Matrix4 {
    let mut result = vec![0.0; 16];
    for i in 0..4 {
      for j in 0..4 {
        result[i * 4 + j] =
          self.elements[i * 4 + 0] * other.elements[0 * 4 + j] +
          self.elements[i * 4 + 1] * other.elements[1 * 4 + j] +
          self.elements[i * 4 + 2] * other.elements[2 * 4 + j] +
          self.elements[i * 4 + 3] * other.elements[3 * 4 + j];
      }
    }
    Matrix4 { elements: result }
  }

  /**
  * Add another Matrix4 to this one.
  * Returns a new Matrix4 instance with the result.
  */
  pub fn add(&self, incoming: &Matrix4) -> Matrix4 {
    let mut result = vec![0.0; 16];
    for i in 0..4 {
      for j in 0..4 {
        result[i * 4 + j] = self.elements[i * 4 + j] + incoming.elements[i * 4 + j];
      }
    }

    Matrix4 { elements: result }
  }

  /**
  * Subtract another Matrix4 from this one.
  * Returns a new Matrix4 instance with the result.
  */
  pub fn subtract(&self, incoming: &Matrix4) -> Matrix4 {
      let mut result = vec![0.0; 16];
      for i in 0..4 {
        for j in 0..4 {
          result[i * 4 + j] = self.elements[i * 4 + j] - incoming.elements[i * 4 + j];
        }
      }

      Matrix4 { elements: result }
    }

  /**
  * Flatten the matrix into a vector of f64.
  */
  pub fn flatten(&self) -> Vec<f64> {
    self.elements.clone()
  }

  /**
  * Get the element at a specific index in the flattened matrix.
  */
  pub fn get_element_at(&self, index: usize) -> Option<f64> {
    let flat = self.flatten();
    if index < flat.len() {
      Some(flat[index])
    } else {      
      None
    }
  }

  /**
  * Check if the matrix is a zero matrix.
  */
  pub fn is_zero(&self) -> bool {
    self.elements.iter().all(|&x| x == 0.0)
  } 

  /**
  * Check if the matrix is an identity matrix.
  */
  pub fn is_identity(&self) -> bool {
    self.elements == vec![
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ]
  }
}
