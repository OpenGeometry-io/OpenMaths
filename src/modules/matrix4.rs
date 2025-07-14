use crate::modules::matrix3::Matrix3;

/**
 * Matrix4 Module
 * 2D Representation of a 4x4 Matrix
 * It's a row-major matrix.
 */
pub struct Matrix4 {
  pub elements: [[f64; 4]; 4],
}

impl Matrix4 {
  /**
  * Create a new Matrix4 with identity elements.
  */
  pub fn new() -> Matrix4 {
    Matrix4 {
      elements: [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
      ],
    }
  }

  pub fn new_with_elements(elements: [[f64; 4]; 4]) -> Matrix4 {
    Matrix4 { elements }
  }

  /**
  * Clone the Matrix4 instance and return a new one.
  */
  pub fn clone(&self) -> Matrix4 {
    Matrix4 {
      elements: self.elements,
    }
  }

  /**
  * Sets the matrix element to create an identity matrix.
  */
  pub fn identity(&mut self) {
    self.elements = [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ];
  }

  /**
  * Determinant for a 4x4 matrix.
  * https://en.wikipedia.org/wiki/Determinant
  */
  pub fn determinant(&self) -> f64 {
    let mut det = 0.0;
    // Calculate the determinant using the first row
    // and the 3x3 minor matrix, then recursively calculate the determinant for each minor.
    for i in 0..4 {
      let mut sub_matrix = [[0.0; 3]; 3];
      for j in 1..4 {
        for k in 0..4 {
          if k < i {
            sub_matrix[j - 1][k] = self.elements[j][k];
          } else if k > i {
            sub_matrix[j - 1][k - 1] = self.elements[j][k];
          }
        }
      }
      det += if i % 2 == 0 { 1.0 } else { -1.0 } * self.elements[0][i] * Matrix3::new_with_elements(sub_matrix).determinant();
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
              let row = if m < i { m } else { m - 1 };
              let col = if n < j { n } else { n - 1 };
              sub_matrix[row][col] = self.elements[m][n];
            }
          }
        }
        adj.elements[j][i] = if (i + j) % 2 == 0 {
          Matrix3::new_with_elements(sub_matrix).determinant()
        } else {
          -Matrix3::new_with_elements(sub_matrix).determinant()
        };
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
    for i in 0..4 {
      for j in 0..4 {
        inv.elements[i][j] = adj.elements[i][j] / det;
      }
    }
    inv
  }

  /**
  * Multiply this matrix with another Matrix4
  * Returns a new Matrix4 instance
  */
  pub fn multiply(&self, other: &Matrix4) -> Matrix4 {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
      for j in 0..4 {
        result[i][j] = self.elements[i][0] * other.elements[0][j]
                     + self.elements[i][1] * other.elements[1][j]
                     + self.elements[i][2] * other.elements[2][j]
                     + self.elements[i][3] * other.elements[3][j];
      }
    }
    Matrix4 { elements: result }
  }

  /**
  * Add another Matrix4 to this one.
  * Returns a new Matrix4 instance with the result.
  */
  pub fn add(&self, incoming: &Matrix4) -> Matrix4 {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
      for j in 0..4 {
        result[i][j] = self.elements[i][j] + incoming.elements[i][j];
      }
    }

    Matrix4 { elements: result }
  }

  /**
  * Subtract another Matrix4 from this one.
  * Returns a new Matrix4 instance with the result.
  */
  pub fn subtract(&self, incoming: &Matrix4) -> Matrix4 {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
      for j in 0..4 {
        result[i][j] = self.elements[i][j] - incoming.elements[i][j];
      }
    }

    Matrix4 { elements: result }
  }

  /**
  * Flatten the matrix into a vector of f64.
  */
  pub fn flatten(&self) -> Vec<f64> {
    self.elements.iter().flat_map(|row| row.iter()).cloned().collect()
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
    self.elements.iter().all(|row| row.iter().all(|&x| x == 0.0))
  } 

  /**
  * Check if the matrix is an identity matrix.
  */
  pub fn is_identity(&self) -> bool {
    self.elements == [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ]
  }
}
