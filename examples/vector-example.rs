use openmaths::Vector3;

fn main() {
  // Create two Vector3 instances
  let mut v1 = Vector3::new(1.0, 2.0, 3.0);
  let v2 = Vector3::new(4.0, 5.0, 6.0);

  // Vector addition
  v1.add(&v2);
  println!("1. Elements of v1 after addition: ({}, {}, {})", v1.x, v1.y, v1.z);

  // Adding a Scalar value to a Vector3
  let mut v3 = Vector3::new(1.0, 3.0, 2.0);
  v3.add_scalar(2.0);
  println!("2. Adding a Scalar to v3, new elements: ({}, {}, {})", v3.x, v3.y, v3.z);

  // Subtracting a Vector3
  let mut v4 = Vector3::new(10.0, 9.0, 8.0);
  v4.subtract(&v2);
  println!("3. Elements of v4 after subtraction: ({}, {}, {})", v4.x, v4.y, v4.z);

  // Subtracting a Scalar value from a Vector3
  let mut v5 = Vector3::new(5.0, 6.0, 7.0);
  v5.subtract_scalar(3.0);
  println!("4. Subtracting a Scalar from v5, new elements: ({}, {}, {})", v5.x, v5.y, v5.z);

  // Cloning a Vector3
  let v6 = v1.clone();
  println!("5. Cloned Vector3 v6: ({}, {}, {})", v6.x, v6.y, v6.z);

  // Zeroing a Vector3
  let mut v7 = Vector3::new(8.0, 9.0, 10.0);
  v7.zero();
  println!("6. Zeroed Vector3 v7: ({}, {}, {})", v7.x, v7.y, v7.z);

  // Copying a Vector3
  let mut v8 = Vector3::new(2.0, 3.0, 4.0);
  v8.copy(&v6);
  println!("7. Copied Vector3 v8 from v6: ({}, {}, {})", v8.x, v8.y, v8.z);

  // Multiplying a Vector3 by a scalar
  let mut v9 = Vector3::new(1.0, 2.0, 3.0);
  v9.multiply_scalar(2.0);
  println!("8. v9 after multiplying by scalar: ({}, {}, {})", v9.x, v9.y, v9.z);

  // Multiply two vectors
  let v10 = Vector3::new(2.0, 3.0, 4.0);
  let v11 = Vector3::new(5.0, 6.0, 7.0);
  let mut v12 = v10.clone();
  v12.multiply(&v11);
  println!("9. v12 after multiplying v10 and v11: ({}, {}, {})", v12.x, v12.y, v12.z);

  // Divide a Vector3 by a scalar
  let mut v13 = Vector3::new(10.0, 20.0, 30.0);
  v13.divide_scalar(2.0);
  println!("10. v13 after dividing by scalar: ({}, {}, {})", v13.x, v13.y, v13.z);

  // Divide two vectors
  let mut v14 = Vector3::new(20.0, 30.0, 40.0);
  let v15 = Vector3::new(2.0, 3.0, 4.0);
  v14.divide(&v15);
  println!("11. v14 after dividing by v15: ({}, {}, {})", v14.x, v14.y, v14.z);

  // Negate a Vector3
  let mut v16 = Vector3::new(1.0, -2.0, 3.0);
  v16.negate();
  println!("12. v16 after negation: ({}, {}, {})", v16.x, v16.y, v16.z);

  // Dot product of two vectors
  let v17 = Vector3::new(1.0, 2.0, 3.0);
  let v18 = Vector3::new(4.0, 5.0, 6.0);
  let dot_product = v17.dot(&v18);
  println!("13. Dot product of v17 and v18: {}", dot_product);

  // Magnitude of a Vector3
  let v19 = Vector3::new(3.0, 4.0, 0.0);
  let magnitude = v19.magnitude();
  println!("14. Magnitude/Length of v19: {}", magnitude);

  // Normalize a Vector3
  let mut v20 = Vector3::new(3.0, 4.0, 0.0);
  v20.normalize();
  println!("15. Normalized v20: ({}, {}, {})", v20.x, v20.y, v20.z);

  // Cross product of two vectors
  let v21 = Vector3::new(1.0, 0.0, 0.0);
  let v22 = Vector3::new(0.0, 1.0, 0.0);
  let v23 = v21.cross(&v22);
  println!("16. Cross product of v21 and v22: ({}, {}, {})", v23.x, v23.y, v23.z);

  // Distance between two vectors
  let v24 = Vector3::new(1.0, 2.0, 3.0);
  let v25 = Vector3::new(4.0, 5.0, 6.0);
  let distance = v24.distance(&v25);
  println!("17. Distance between v24 and v25: {}", distance);

  // Apply a transformation matrix with translation at 2 in x axis, to a vector
  let mut v26 = Vector3::new(1.0, 2.0, 3.0);
  let matrix = openmaths::Matrix4::set(
    1.0, 0.0, 0.0, 2.0,
    0.0, 1.0, 0.0, 5.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
  );
  v26.apply_matrix4(matrix);
  println!("18. v26 after applying transformation matrix: ({}, {}, {})", v26.x, v26.y, v26.z);
}
