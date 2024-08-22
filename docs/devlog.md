# Devlog

### 22nd August 2024

#### More on Match 

- Match works with tuples as well, either all the elements can be tested for a match or even some elements in a partocular order can be tested as well.
```rust
match vector {
  (0, ..) => println!("First Element is 0"),
  (.., 0) => println!("Last Element is 0"),
  (0, 0, 0) => ("Zero Vector"),
  _ => println!("Default Case, don't bind to anything"),
}
```

- Match also works with array and can be tested for a value or can be indexed as well. The value needs to be mentioned explicitly and the index can be defined like variables and the values are binded to it
```rust
match vector {
  [0, y, z] => println!("First element is 0, y is {} and z is {}", y, z),
  [x, 0, ..] => println!("x is {}, Second element is 0", x),
  [x, rest @ ..] => println!("x is {}, rest of the elements are {:?}", x, rest),
}
```
- *@* creates a slice of remaining elements and stores it inside `rest`

- *enums* can be tested using match as well
- more on **binding** later

#### Functions

- Declared using *fn* keyword, return type specified by **->**
- There is a unique type of functions as well, called *associated* functions. They can be associated to a particular type. I like them!
```rust
struct Point3D {
  x: f64,
  y: f64,
  z: f64
}
  
impl Point3D {
  // called as a contructor
  fn set(x: f64, y: f64, z: f64) -> Point3D{
    Point3D { x: x, y: y, z: z}
  }

  // called as a method
  fn add (&self, other: &Point3D) -> Point3D {
    Point3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
  }
}
```

### 21st August 2024

#### Loop

- Loop is created using *loop* keyword, `break` keyword will break the loop and `continue` skips the rest of the implementation and starts over
- Loops can be nested and should be anotated with `'` with a name to it, e.g. `'outer: loop {}`
- To break or continue the nested loop it can be done like this, `break 'outer`
- Loop can return value, if the value is attached after the break keyword. e.g. `break result`

#### While

- A while loop can be used until the condition is satisfied
```rust
while value < 10 {
  println!("number is {}", value)
  value += 1
}
```

#### For Loops

- `for in` can be used to iterate through *Iterator*, it is not anywhere close to TS
- The iterator initial value and final value should contain `..` notation.
```rust
for n in 1..10 {
  println!("number is {}", n)
}
```
- In the above case loop will run until `n` is 9. To include 10 as well, loop should be like this `for n in 1..=10`

#### Match like Switch

- Works just like any other switch statement
- Match can be used as an expression as well and used later on. e.g. `let switchStatement = match incomeTax { .... } `

### 20th August 2024

#### TryFrom and TryInto

* Useful when doing type conversion that have some chances of failing and need special handling
* _From_ and _Into_ are for perfect conversions
* **TryFrom** and **TryInto** checks if the conversion can go bad, it is always better to use **TryFrom** and **TryInto** rather than relying on From (more safe)

#### String Operations

* String can be parsed using `"5".parse().unwrap()`
* type can be converted to string using e.g. `rectangle.to_string()`

#### Expressions

* Blocks are expressions too, the last statement is a return value of that block

```rust
let y = {
  let x = 1;
  let j = 2;
  x + j
}

println!("{}", y); // 3
```

#### Flow

* if-else is similar. The condition doesn't need to be surronded by **()**
* All blocks return a value and the last expression does the same here

```rust
let num = 32;
let num_type = 
  if num/2 == 0 {
    println!("num is even");
    num * 10
  } else {
    println!("num is odd");
    num * 11
  };

println!("{}", num_type); // 3200
```

### 19th August 2024

#### Casting Continued..

* Casting works in some cases and not always, e.g. it is not possible to cast float to char
* Cast needs to be used wisely

#### Literals

* Numerical literals can be type annoted, the type needs to be added after the value
* e.g. _let i = 3iu8_

#### Inference & Aliasing

* Type inference can check for the array elements and infer the type based upon it's use
* Types can be renamed using aliasing method, for e.g. _type KG = u32_
* When using type aliasing, the types should _UpperCamelCase_ names to prevent compiler warnings

#### Conversion

* **From** trait allows the way to create itself from another type
* **Into** is similar to From but it doesn't take value, rather called upon the variable
* **From** and **Into** are interchangeable

### 18th August 2024

#### More on Custom Types

* `const` and `static` can be used in Rust the same way done in another langauges like TS

#### Variable Bindings

* Rust provides _type safety_ via statis typing, in most cases compiler infer type based on context
* By default, the variables are non-mutable in Rust, to create a mutable(ability to change) variable you need to use `mut` keyword.

```rust
let age = 20
age = 21 // ❌ error

let mut age = 20
age = 21 // ✅ works
```

* Shadow binding is allowed when a variable is declared in different scopes
* Redeclaration is allowed as well, this shadows the previous bindings

#### Casting

* Explicit Casting is possible using `as` keyword. _let pi\_int = pi\_decimal as i32_

### 17th August 2024

#### Strutctures

* Three types - Tuples, C type structs and Unit struct which are fieldless
* Structs can be destructured as well `let Date { year, month, day, day_of_week } = date;`
* Tuples and Struct appear to be same but the way they are accessed are different, structs cannot be indexed. There can be confusions when using tuple, I **might** avoid it as of now

#### Enums

* Allows the creation of type which can have one or more variant inside it
* It is recommended to keep enums in uppercase
* Didn't understood this well, need to come back later to this [example](https://doc.rust-lang.org/rust-by-example/custom\_types/enum/testcase\_linked\_list.html)

### 16th August 2024

#### Publishing to Rust Crates

* `openmath` seems to be already take, renaming the project to `openmaths`. I'm not obssesed with names, users don't care about name, they need something that works(IMO)
* Also, `Crates`- Package Registery for Rust, doesn't allow scoped crates, so the crate(package) name needs to be unique
* No sane person would publish a package in the first weeks of development, there are two things, I can't contain my excitement and second my aim is build and publish the library so I need to check the feasibility as well.
* Before you can push packages to crate, you have to login - `cargo login`
* Push the latest changes to `git` and then `cargo package` - creates a `.crate` file with source code
* License and Description are mandatory if you are building crates
* To publish the package - `cargo publish`

#### Basic Know-How

* Variables can be _type annotated_, also rust can infer types from context. Make sure to type annotate everything that is being used
* `i32` is _signed 32-bit integer_ and `u32` is _unsigned 32-bit integer_
* Why type annotation is important? - `println!("1 - 2 = {}", 1u32 - 2);` when we run this, a compile error is thrown and it is easier to catch silly issues like this early on!
* `_` using an _underscore_ can improve readibility when dealing with large numbers, `println!("One million is written as {}", 1_000_000u32);` this will omit an underscore and `100000` will be printed to the console
* Camel Case notation is discouraged when creating variables, instead using an _underscore_ is preferrred. Coming from JS this feels a little daunting but I hope it grows on me
* _let numArr_ ⚠️
* _let num\_arr_ ✅

#### Tuples

* Tuples is a collection of data, it can be of similar data type or mixed
* It can be passed as a function parameter and can be used to return data as well
* It can be destructured like what we do with javascript objects
* Values can be extracted from tuple using index, e.g. `let tup = ('i', 'n', 'd')` when printed using `println!("{}", tup.1)` will print _n_
* Tuples can have multiple tuple within them - _nested tuples_
* Tuple can be printed, but it does not print more than 12 tuple memebers
* If a tuple contains a single value it should be separated by a comma, else rust considers it as a integer. `(5u32,)` - way to write _single element tuple_

#### Return of the function

* I found one thing strange, rust doesn't force you to mention _return_ keyword unlike JS
* Don't let being perfectly idiomatic stop you from writing what you believe to be good code, I will use **return**

#### Array and Slices

* Similar to other languages, _array_ is a collection of the same type **T**, stored in contiguous memory. Length is known at compile time
* `let xs: [i32; 5]` - here _i32_ is the type of elements present inside the _xs_ array and _5_ states the number of elements
* Length of an array can be fethced using `array.len()`
* Memory size can be computed using `mem::size_of_val(&xs))`
* Slices seem to be complex, need to look more deep into it!

#### Custom

* Custom types are created using `struct` and `enum`
* Constants can be created using `const` and `static` keyword

### 15th August 2024

#### Baby Steps

* Started follwing [rust by example](https://doc.rust-lang.org/rust-by-example/)
* Cargo.toml file contains the dependencies, every dependecy is similar to a package/library in other programming languages
* To add dependency - `cargo add packageName`
* Using the package - `use packageName`
* disable unused code warning - `#[allow(dead_code)]`, don't use it really - always keep your code clean

### 14th August 2024

#### Rust vs C++

* I have came to a conclusion that I need a robust math library for the Kernel Operations
* When I went to look for how these Math Libraries are structured, I found that the best solution these day is to use Native Code + WebAssembly.
* But I struggled to decide, with so many options, two langauges are mentioned the most `C++` and `Rust`
* I'm no where a expert what is best, and then I thought why not create a package using both?
* This is not going to hurt me, surely I will have to invest double time, but I'm confident that this learning wouldn't go away!

#### Installing Rust and Project Setup

* On a mac, run this command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Verify the installation by `cargo --version`, remember to close your terminal after the rust is installed
* Create new project - `cargo new projectName`
