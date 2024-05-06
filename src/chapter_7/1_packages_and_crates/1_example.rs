// Library crates don’t have a main function, and they don’t compile to an executable. 
// Instead, they define functionality intended to be shared with multiple projects. 
// For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers. 
// Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".

// A package is a bundle of one or more crates that provides a set of functionality. 
// A package contains a Cargo.toml file that describes how to build those crates. 
// Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. 
// The Cargo package also contains a library crate that the binary crate depends on. 
// Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.