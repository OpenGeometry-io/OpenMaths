# 🎥 Devlog

## 16th August 2024

### Publishing to Rust Crates
- `openmath` seems to be already take, renaming the project to `openmaths`. I'm not obssesed with names, users don't care about name, they need something that works(IMO)
- Also, `Crates`- Package Registery for Rust, doesn't allow scoped crates, so the crate(package) name needs to be unique 
- No sane person would publish a package in the first weeks of development, there are two things, I can't contain my excitement and second my aim is build and publish the library so I need to check the feasibility as well.
- Before you can push packages to crate, you have to login - `cargo login`
- Push the latest changes to `git` and then `cargo package` - creates a `.crate` file with source code 

## 15th August 2024

### Baby Steps
- Started follwing [rust by example](https://doc.rust-lang.org/rust-by-example/)
- Cargo.toml file contains the dependencies, every dependecy is similar to a package/library in other programming languages
- To add dependency - `cargo add packageName`
- Using the package - `use packageName`
- disable unused code warning - `#[allow(dead_code)]`, don't use it really - always keep your code clean

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
