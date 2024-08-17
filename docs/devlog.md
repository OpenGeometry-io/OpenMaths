# 🎥 Devlog

### 17th August 2024

#### Strutctures
- Three types - Tuples, C type structs and Unit struct which are fieldless
- Structs can be destructured as well `let Date { year, month, day, day_of_week } = date;`
- Tuples and Struct appear to be same but the way they are accessed are different, structs cannot be indexed. There can be confusions when using tuple, I **might** avoid it as of now

#### Enums
- Allows the creation of type which can have one or more variant inside it
- It is recommended to keep enums in uppercase

### 16th August 2024

#### Publishing to Rust Crates
- `openmath` seems to be already take, renaming the project to `openmaths`. I'm not obssesed with names, users don't care about name, they need something that works(IMO)
- Also, `Crates`- Package Registery for Rust, doesn't allow scoped crates, so the crate(package) name needs to be unique 
- No sane person would publish a package in the first weeks of development, there are two things, I can't contain my excitement and second my aim is build and publish the library so I need to check the feasibility as well.
- Before you can push packages to crate, you have to login - `cargo login`
- Push the latest changes to `git` and then `cargo package` - creates a `.crate` file with source code 
- License and Description are mandatory if you are building crates
- To publish the package - `cargo publish`

#### Basic Know-How
- Variables can be *type annotated*, also rust can infer types from context. Make sure to type annotate everything that is being used
-  `i32` is *signed 32-bit integer* and `u32` is *unsigned 32-bit integer*
- Why type annotation is important? - `println!("1 - 2 = {}", 1u32 - 2);` when we run this, a compile error is thrown and it is easier to catch silly issues like this early on!
- `_` using an *underscore* can improve readibility when dealing with large numbers, `println!("One million is written as {}", 1_000_000u32);` this will omit an underscore and `100000` will be printed to the console
- Camel Case notation is discouraged when creating variables, instead using an *underscore* is preferrred. Coming from JS this feels a little daunting but I hope it grows on me
- *let numArr* ⚠️
- *let num_arr* ✅

#### Tuples
- Tuples is a collection of data, it can be of similar data type or mixed
- It can be passed as a function parameter and can be used to return data as well
- It can be destructured like what we do with javascript objects
- Values can be extracted from tuple using index, e.g. `let tup = ('i', 'n', 'd')` when printed using `println!("{}", tup.1)` will print *n*
- Tuples can have multiple tuple within them - *nested tuples*
- Tuple can be printed, but it does not print more than 12 tuple memebers
- If a tuple contains a single value it should be separated by a comma, else rust considers it as a integer. `(5u32,)` - way to write *single element tuple*

#### Return of the function
- I found one thing strange, rust doesn't force you to mention *return* keyword unlike JS
- Don't let being perfectly idiomatic stop you from writing what you believe to be good code, I will use **return**

#### Array and Slices
- Similar to other languages, *array* is a collection of the same type **T**, stored in contiguous memory. Length is known at compile time
- `let xs: [i32; 5]` - here *i32* is the type of elements present inside the *xs* array and *5* states the number of elements
- Length of an array can be fethced using `array.len()`
- Memory size can be computed using `mem::size_of_val(&xs))`
- Slices seem to be complex, need to look more deep into it!

#### Custom
- Custom types are created using `struct` and `enum`
- Constants can be created using `const` and `static` keyword

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
