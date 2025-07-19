use openmaths::Matrix4;

fn main() {
    // Create a new Matrix4 instance
    let mut m1 = Matrix4::new();
    println!("1. Initial Matrix4 m1: {:?}", m1.elements());

    // Set m1 to identity matrix
    m1.identity();
    println!("2. Identity Matrix4 m1: {:?}", m1.elements());

    // Create another Matrix4 instance with specific elements
    let m2 = Matrix4::set(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );

    println!("3. Matrix4 m2: {:?}", m2.elements());

    // // Multiply two matrices
    // let m3 = m1.clone().multiply(&m2);
    // println!("4. Result of multiplying m1 and m2: {:?}", m3.elements);

    // // Add two matrices
    // let m4 = m1.clone().add(&m2);
    // println!("5. Result of adding m1 and m2: {:?}", m4.elements);

    // // Subtract two matrices
    // let m5 = m2.clone().subtract(&m1);
    // println!("6. Result of subtracting m1 from m2: {:?}", m5.elements);

    // Determinant of a matrix m6
    let m6 = Matrix4::set(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let det = m6.determinant();
    println!("7. Determinant of m6: {}", det);

    // // Adjugate of a matrix
    // let adj = m6.adjucate();
    // println!("8. Adjugate of m6: {:?}", adj.elements);

    // // Inverse of a matrix
    // let m7 = m6.clone().inverse();
    // println!("9. Inverse of m6: {:?}", m7.elements);

    // // Check if the inverse is correct by multiplying m6 with its inverse
    // let m6_cloned = m6.clone();
    // let m6_inverse_multiplied = m6_cloned.multiply(&m7);
    // println!("10. Result of multiplying m6 with its inverse: {:?}", m6_inverse_multiplied.elements);
    
    // // Check if the inverse multiplication gives identity matrix
    // let identity_check = m6_inverse_multiplied.is_identity();
    // println!("11. Is the result an identity matrix? - {}", identity_check);
}