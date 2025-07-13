
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
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn new_with_elements(elements: [[f64; 4]; 4]) -> Matrix4 {
    Matrix4 { elements }
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

  pub fn add(&self, incoming: &Matrix4) -> Matrix4 {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
      for j in 0..4 {
        result[i][j] = self.elements[i][j] + incoming.elements[i][j];
      }
    }

    Matrix4 { elements: result }
  }

  pub fn flatten(&self) -> Vec<f64> {
    self.elements.iter().flat_map(|row| row.iter()).cloned().collect()
  }
}
