fn main() {
    // because of "!", rust calling macro instead of normal function.
    // macros dont always follow the same rules as functions
    println!("hello world");
}

// explanation
// rust program must have main function as its entry point