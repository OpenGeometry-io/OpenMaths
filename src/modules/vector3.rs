
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 { x, y, z }
  }

  /**
  * Add the elements of another vector to this one.
  */
  pub fn add(&mut self, other: &Vector3) {
    self.x = self.x + other.x;
    self.y = self.y + other.y;
    self.z = self.z + other.z;
  }

  /**
  * Add a scalar value to each element of the vector.
  */
  pub fn add_scalar(&mut self, scalar: f64) {
    self.x += scalar;
    self.y += scalar;
    self.z += scalar;
  }

  /**
  * Subtract the elements of another vector from this one.
  */
  pub fn subtract(&mut self, other: &Vector3) {
    self.x = self.x - other.x;
    self.y = self.y - other.y;
    self.z = self.z - other.z;
  }

  /**
  * Subtract a scalar value from each element of the vector.
  */
  pub fn subtract_scalar(&mut self, scalar: f64) {
    self.x -= scalar;
    self.y -= scalar;
    self.z -= scalar;
  }

  /**
  * Clone the Vector3 instance and return a new one.
  */
  pub fn clone(&self) -> Vector3 {
    Vector3 {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }

  /**
  * Set the current vector elements to zero.
  */
  pub fn zero(&mut self) {
    self.x = 0.0;
    self.y = 0.0;
    self.z = 0.0;
  }

  /**
  * Copy the elements from another Vector3 instance. 
  */
  pub fn copy(&mut self, other: &Vector3) {
    self.x = other.x;
    self.y = other.y;
    self.z = other.z;
  }

  /**
  * Multiply the vector by a scalar value.
  */
  pub fn multiply_scalar(&mut self, scalar: f64) {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
  }

  /**
  * Multiply the vector by another Vector3 instance element-wise.
  */
  pub fn multiply(&mut self, other: &Vector3) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
  }

  /**
  * Divide the vector by another Vector3 instance element-wise. 
  */
  pub fn divide(&mut self, other: &Vector3) {
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
  }

  /**
  * Divide the vector by a scalar value.
  */
  pub fn divide_scalar(&mut self, scalar: f64) {
    self.x /= scalar;
    self.y /= scalar;
    self.z /= scalar;
  }

  /**
  * Invert the vector by negating the vector elements.
  */
  pub fn negate(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
  }

  /**
  * Calculate the dot product of this vector with another Vector3 instance.
  */
  pub fn dot(&self, other: &Vector3) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  /**
  * Calculate the magnitude (length) of the vector.
  */
  pub fn magnitude(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  /**
  * Calculate the length of the vector.
  * Since length is equivalent to magnitude, this method is an alias.
  */
  pub fn length(&self) -> f64 {
    self.magnitude()
  }

  /**
  * Normalize the vector
  */
  pub fn normalize(&mut self) {
    let mag = self.magnitude();
    if mag > 0.0 {
      self.divide_scalar(mag);
    }
  }

  /**
  * Calculate the cross product and return the result as a new Vector3
  */
  pub fn cross(&self, other: &Vector3) -> Vector3 {
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }

  /**
  * Calculate the distance between this vector and another Vector3 instance.
  * Return the distance as a f64 value in Euclidean space.
  */
  pub fn distance(&self, other: &Vector3) -> f64 {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    let dz = self.z - other.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
  }

}
