fn main() {
    let s = String::from("World");
    let mut b = s.clone();
    // scoped ownership
    {
        let s = "hello";
        println!("{s}");
    }
    // s variable will be no longer valid in here (scope is over);
    b = String::from("hell");
    println!("{}", s);
    // borrowing ownership
    let str = String::from("hello");
    takes_ownership(str); // this function takes ownership of "str" from the main (because String doesnt implment the copy trait)
    // println!("{}", str); // cannot access "str" because the ownership has been takes by previous function
    let x = 5;
    makes_copy(x); // this function will copy the value of the integer. because integer implement copy trait
    println!("{}", x); // still can use "x" variable
    let s1 = gives_ownership(); // gives_ownership() will give the ownership of the String that this function return to the variable
    let s2 = String::from("my string");
    let s3 = takes_and_gives_back(s2); // takes_and_gives_back() function will takes the ownership of s3, and then give the ownership again to the variable.
    // we can do something with s1 (because the ownership already given to this scope(main))
    println!("{}", s1);
    // we also can still do something with s3 because the ownership is returned to this scope (but not with s2 because the ownership is moved to s3)
    // println!("{}",s2); // will error
    println!("{}", s3); // will do

    // referencing the ownership
    // the idea is not directly give the ownership to the calling function/variable. its give the reference to the one who owning the ownership.
    let mut my_string = String::from("my string"); // "my_string" enter the scope
    let len = calculate_length(&my_string); // we give the reference of the ownership to the function, so the we are not giving the ownership to the calling function.
    // we still can access "my_string" after calling "calculate_length()"
    println!("the length of {my_string} is {len}");
    let word = first_word(&my_string);
    // my_string.clear();
    println!("{word}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("your string now");
    return some_string;
}

fn takes_and_gives_back(some_string: String) -> String {
    return some_string;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
