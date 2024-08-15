# 🎥 Devlog

### 15th August 2024

#### Baby Steps

* Started follwing [rust by example](https://doc.rust-lang.org/rust-by-example/)
* Cargo.toml file contains the dependencies, every dependecy is similar to a package/library in other programming languages
* To add dependency - `cargo add packageName`
* Using the package - `use packageName`
* disable unused code warning - `#[allow(dead_code)]`, don't use it really - always keep your code clean
*

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
