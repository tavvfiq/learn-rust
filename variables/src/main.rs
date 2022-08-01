use std::io;

// const is always immutable.
// the type must be annotated (in this case is u32)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    // variable is immutable by default
    // let x = 5;
    // lets create variable with keyword "mut"
    let mut x = 5;
    println!("The value of x is: {x}");
    // will be error on re-assignment of variable
    x = 6;
    println!("The value of x is: {x}");
    // shadowing variable
    let x = 5;
    // above variable will be shadowed by this re-declaration
    let x = x + 1;
    {
        // this x will have shadow above x with this scope lifetime.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // x will be back to 6 because the above scope is end.
    println!("The value of x is: {x}");
    data_types_numeric();
    data_types_bool();
    data_types_char();
    data_types_compound();
    expression();
    sum(x, x);
    control_flow();
}

fn data_types_numeric() {
    // integer
    let _unsigned8: u8 = 1;
    let _unsigned16: u16 = 1;
    let _unsingned32: u32 = 1;
    let _unsigned64: u64 = 1;
    let _signed8: i8 = 1;
    let _signed16: i16 = 1;
    let _signed32: i32 = 1;
    let _signed64: i64 = 1;

    // float
    let _float32: f32 = 1.0;
    let _float64: f64 = 1.0;

    // numeric operations
    // addition
    let _sum = _unsigned8 + _unsigned8;
    let _sum = _signed16 + _signed16;
    let _sum = _float32 + _float32;
    // subtraction
    let _diff = _unsigned16 - _unsigned16;
    let _diff = _signed16 - _signed16;
    let _diff = _float32 - _float32;
    // multiplication
    let _mul = _unsigned16 * _unsigned16;
    let _mul = _signed16 * _signed16;
    let _mul = _float32 * _float32;
    // division
    let _div = _unsigned16 / _unsigned16;
    let _div = _signed16 / _signed16;
    let _div = _float32 / _float32;
    // reminder
    let _rem = _unsigned16 % _unsigned16;
    let _rem = _signed16 % _signed16;
    let _rem = _float32 % _float32;
}

fn data_types_bool() {
    let _t = true;
    let _f: bool = false;
}

fn data_types_char() {
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}

fn data_types_compound() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // accessing tuple with period
    let first_element = tup.0;
    println!("The value of first element is: {first_element}");

    // array
    // every element in an array have same type
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let b: [f32; 5] = [1.2, 1.3, 1.4, 1.5, 1.6];
    // init an array with the same value.
    // first is the value, second is the length of the array
    let c = [99; 5];
    // accessing array element with index
    let first = a[0];
    let second = a[1];
    // invalid_accessing_array_element();
}

fn invalid_accessing_array_element() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}

fn expression() {
    // expression with curly bracket
    let y = {
        // add semicolon so this line after is a statement
        let x = 3;
        // not adding any semicolon so this line after is an expression (will be evaluated)
        x + 1
    };
    println!("The value of y is: {y}");
}

// function with return value
fn sum(a: i32, b: i32) -> i32 {
    // return a + b;
    // can implicitly return if last statement is not ended with semicolon.
    // (because it is evaluated as an expression)
    a + b
}

fn control_flow() {
    // if statement
    let x = 0;
    if x > 0 {
        println!("Wohooo");
    } else {
        println!("nope");
    }
    // the condition must be evaluated to bool
    // will error
    // if x {
    //     println!("x is a number");
    // }
    // using if in a let statement
    let y = if x > 0 { x + 1 } else { 
        let x = x + 2; 
        x
    };
    println!("The value of y is: {y}");

    // looping
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // labelling loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // conditional loop with while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index > 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
    // more concise way
    for element in a {
        println!("the value is {element}");
    }
    // looping in a range of number
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
