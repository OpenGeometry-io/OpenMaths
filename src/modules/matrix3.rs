pub struct Matrix3 {
  pub elements: [[f64; 3]; 3],
}

impl Matrix3 {
  /**
  * Create a new Matrix3 with identity elements.
  */
  pub fn new() -> Matrix3 {
    Matrix3 {
      elements: [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn new_with_elements(elements: [[f64; 3]; 3]) -> Matrix3 {
    Matrix3 { elements }
  }

  /**
  * Clone the Matrix3 instance and return a new one.
  */
  pub fn clone(&self) -> Matrix3 {
    Matrix3 {
      elements: self.elements,
    }
  }

  /**
  * Sets the matrix element to create an identity matrix.
  */
  pub fn identity(&mut self) {
    self.elements = [
      [1.0, 0.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 0.0, 1.0],
    ];
  }

  /**
  * Determinant for a 3x3 matrix.
  * https://en.wikipedia.org/wiki/Determinant
  */
  pub fn determinant(&self) -> f64 {
    self.elements[0][0] * (self.elements[1][1] * self.elements[2][2] - self.elements[1][2] * self.elements[2][1]) -
    self.elements[0][1] * (self.elements[1][0] * self.elements[2][2] - self.elements[1][2] * self.elements[2][0]) +
    self.elements[0][2] * (self.elements[1][0] * self.elements[2][1] - self.elements[1][1] * self.elements[2][0])
  }

  pub fn adjucate(&self) -> Matrix3 {
    let mut adj = Matrix3::new();
    for i in 0..3 {
      for j in 0..3 {
        let mut sub_matrix = [[0.0; 2]; 2];
        for m in 0..3 {
          for n in 0..3 {
            if m != i && n != j {
              let row = if m < i { m } else { m - 1 };
              let col = if n < j { n } else { n - 1 };
              sub_matrix[row][col] = self.elements[m][n];
            }
          }
        }
        adj.elements[j][i] = if (i + j) % 2 == 0 {
          sub_matrix[0][0] * sub_matrix[1][1] - sub_matrix[0][1] * sub_matrix[1][0]
        } else {
          -(sub_matrix[0][0] * sub_matrix[1][1] - sub_matrix[0][1] * sub_matrix[1][0])
        };
      }
    }
    adj
  }

  pub fn inverse(&self) -> Option<Matrix3> {
    let det = self.determinant();
    if det == 0.0 {
      return None; // No inverse exists
    }
    let adj = self.adjucate();
    let mut inv = Matrix3::new();
    for i in 0..3 {
      for j in 0..3 {
        inv.elements[i][j] = adj.elements[i][j] / det;
      }
    }
    Some(inv)
  }
}
  