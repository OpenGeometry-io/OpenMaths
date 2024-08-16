/*
    * OpenMaths - A Rust library for OpenMaths
    * Copyright (C) 2024 OpenMaths Development Team
    *
    * @file src/main.rs
    * @brief Main file for the OpenMaths Rust library
    * @date 2024-08-15
    *
    * @author Vishwajeet Mane
    * @version 0.1
    * @date 2024-01-01
    *
    * @todo Everything
    * @bug None
    * @warning None
*/

use std::fmt;

const PACKAGE_NAME: &str = "openmaths";
const PACKAGE_VERSION: &str = "0.0.1";

struct Matrix( (i32, i32), (i32, i32) );

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Matrix( (a, b), (c, d) ) = *self;
        write!(f, "({} {})\n({} {})", a, b, c, d)
    }
}

fn main() {
    // let stdout = std::io::stdout();
    // let message = String::from("Hello, World!");
    // let width = message.chars().count();

    // let mut writer = std::io::BufWriter::new(stdout.lock());
    // ferris_says::say(&message, width, &mut writer).unwrap();

    println!("{0}::{1} up and running", PACKAGE_NAME, PACKAGE_VERSION);

    let basic_matrix:Matrix = Matrix( (1, 2), (3, 4) );
    
    println!("Basic Matrix:\n{}", basic_matrix);

    println!("Transpose of Basic Matrix:\n{}", transpose(basic_matrix));

    let num_arr:[i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {}", num_arr[2]);

    analyze_slice(&num_arr);

    println!("Planned Revenue Target ${} in Q4 2025", 10_000_000u32);
}

fn transpose( matrix: Matrix ) -> Matrix {
    let Matrix( (a, b), (c, d) ) = matrix;
    return Matrix( (a, c), (b, d) )
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

// fn add( a: i32, b: i32 ) -> i32 {
//     return a + b;
// }
