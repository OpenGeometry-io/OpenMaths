use openmaths::openmaths;

fn main() {
    println!("Hello, world!");
    let v1 = openmaths::Vector3D::create(1.0, 2.0, 3.0);
    let v2 = openmaths::Vector3D::create(1.0, 2.0, 3.0);
    let v3 = v1.add(&v2);
    println!("v3.x: {}, v3.y: {}, v3.z: {}", v3.x, v3.y, v3.z);
}
