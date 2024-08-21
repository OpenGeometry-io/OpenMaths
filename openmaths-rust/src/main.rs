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

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

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
    print!("---------------------------------\n");
    struct_example();
    print!("---------------------------------\n");
    enum_example();
    print!("---------------------------------\n");
    mutability_example();
    print!("---------------------------------\n");
    casting_example();
    print!("---------------------------------\n");
    conversion_example();
    print!("---------------------------------\n");
    from_example();
    print!("---------------------------------\n");
    into_example();
    print!("---------------------------------\n");
    try_from_example();
    print!("---------------------------------\n");
    rectangle_example();
    print!("---------------------------------\n");
    flow_control_example();

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


// Struct Example
// Date - 17th August 2024
struct Date {
    year: i32,
    month: i32,
    day: i32,
    day_of_week: String
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{} ({})", self.month, self.day, self.year, self.day_of_week)
    }
}

fn struct_example() {
    let day = 17;
    let day_of_week = "Saturday";
    let month = 08;
    let year = 2024;

    let date = Date {
        month,
        day,
        day_of_week: String::from(day_of_week),
        year,
    };

    // If you directly print the struct, it will give an error
    // The error is because the struct does not implement the Display trait
    println!("Date: {}", date);

    // Desctructuring the struct
    let Date { day_of_week, year, month, day } = date;
    println!("Day is {}", day_of_week);
}


// Enum Example
// Date - 17th August 2024

enum Shape {
    // Enum with no data
    ShapeLoaded,
    
    // Enum with data
    ShapeName(String),

    // Struct-like enum
    Center { x: f64, y: f64 }
}

fn inspect_shape(shape: Shape) {
    match shape {
        Shape::ShapeLoaded => println!("Shape Generated"),
        Shape::ShapeName(name) => println!("Shape Name: {}", name),
        Shape::Center { x, y } => println!("Center at ({}, {})", x, y) 
    }
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn enum_example() {
    let is_shape_loaded = Shape::ShapeLoaded;
    let shape = Shape::ShapeName(String::from("Circle"));
    let center = Shape::Center { x: 0.1, y: 0.2 };

    inspect_shape(is_shape_loaded);
    inspect_shape(shape);
    inspect_shape(center);

    println!("Red: #{:06x}", Color::Red as i32);
}

// Mutability Example
// Date - 18th August 2024

fn mutability_example() {
    let mut price = 100;

    {
        let price = 200;
        // compile error because price is immutable in this scope
        // price = price;

        println!("Price in inner scope: {}", price);
    }

    price = 300;
    println!("Price in outer scope: {}", price);
}

// Casting Example
// Date - 18th August 2024

fn casting_example() {
    let pi:f64 = 3.14159265359_f64;
    println!("Pi in decimal: {}", pi);

    let pi_int = pi as i32;
    println!("Pi in integer: {}", pi_int);
}

// Conversion Example
// Date - 19th August 2024

fn conversion_example() {
    let str = "Hello";
    let string = String::from(str);
    print!("String: {}", string);
}

// From
use std::convert::From;
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn from_example() {
    let point = Number::from(23);
    println!("Point: {}", point.value);
}

// Into
struct Det{
    value: i32,
}

impl Into<Det> for i32 {
    fn into(self) -> Det {
        Det { value: self }
    }
}

fn into_example() {
    let int = 5;
    let det: Det = int.into();
    println!("Det: {:?}", det.value);
}

// TryFrom and TryInto
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct OddNumber(i32);

impl TryFrom<i32> for OddNumber {
    type Error = ();
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 != 0 {
            Ok(OddNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from_example() {
    assert_eq!(OddNumber::try_from(3), Ok(OddNumber(3)));
    assert_eq!(OddNumber::try_from(2), Err(()), "2 is not an odd number");
}

// To and From Strings
// Date - 20th August 2024
struct Rectangle {
    width: u32,
    height: u32,
    area: u32
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle ({} x {}) with Area: {}", self.width, self.height, self.area)
    }
}

fn rectangle_example() {
    let width = 30;
    let height = 50;

    let area = width * height;

    let rectangle = Rectangle { width, height, area };

    println!("{}", rectangle.to_string());

    // parsing string to integer
    let rectangle_perimeter = "100".parse::<i32>().unwrap();
    println!("Rectangle Perimeter: {}", rectangle_perimeter);
}

// Flow Control
fn flow_control_example () {
    let num = 32;
    let num_type = 
    if num%2 == 0 {
        println!("num is even");
        num * 10
    } else {
        println!("num is odd");
        num * 11
    };

    println!("{}", num_type);



    let number = 1;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}

