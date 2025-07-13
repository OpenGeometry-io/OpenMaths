use openmaths::Vector3;
use openmaths::Matrix4;

fn main() {
  // Create two Vector3 instances
  let mut v1 = Vector3::new(1.0, 2.0, 3.0);
  let v2 = Vector3::new(4.0, 5.0, 6.0);

  // Vector addition
  v1.add(&v2);
  println!("Elements of v1 after addition: ({}, {}, {})", v1.x, v1.y, v1.z);

  // Create a Matrix4 (identity)
  let m = Matrix4::new();
  let elements = m.flatten();
  println!("Matrix4 elements: {:?}", elements);
}