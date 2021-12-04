use std::io;

fn main() {
    // create an empty string
    // the input is kinda a smart pointer
    // as it will be de-allocate the
    // string data from the heap
    // when the input variable is
    // out of scope (function exits)
    let mut input = String::new();

    // we can have any number of immutable references
    // or only one mutable reference
    // once a mutable reference is used,
    // we can't use reference after it
    // but we can use immutable references before mutable reference is used
    // this techniques prevents what is called "data races? :D"
    let x = & input;
    let y = &input;
    println!("{}, {}", x, y);

    // we are borrowing  input, without moving it's ownership to some_fun
    // this will stop some_fun from de-allocating it when it exits
    some_fun(&mut input);
    // read_line expects a string variable to be passed
    io::stdin().read_line(&mut input);

    let mut weight = calculate_weight_on_mars(77.0);
    println!("Weight on Mars is: {}kg", weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    // rust returns the last statement, (without a ;)
    (weight / 9.81) * 3.711
}

// & means the paramater is borrowing a reference
// to a variable
// which will not move the ownership of that parameter
// to the function, it will borrow it instead

// also, references are immutable by default,
// unless perceeded witha mut in as a parameter
// and as an argument
fn some_fun(s: &mut String) {
    s.push_str("x");
}

/*

1- Each value in rust is owned by a variable

2- When the owner goes out of scope (wne function exits), the value
will be de-allocated

3- There can only be ONE owner at a time


let mut x = 10
let mut y = x

here, the 10 ownership is moved to y
and x is no longer a valid variable

this is beacuse String::new() is a complex type, which gets stored at heap,
cause it's size is not known at compile time
as stack only stores variables that have size
known  at compile time

for example,  le a = 10
a here will be stored at the stack, not the heap


when a variable is stored at the stack, like let a = 5
 a here stores the actual 5, which is not a pointer

unlike for example, if we have let s = String::new()
s here is a pointer to the string value in the heap


the same applies to functions,
watch video 8 - ownership
*/

/*
 fn main() {
    let  x= String::new();
    let  y = x;

    println!("{}", y);
}
 */


//  borrowing is a way to access the data in the heap
// this can be done with:
// . immutable references: & (read-only) for the value in the heap
// . mutable references: &mut (read-and-write) for the value in the heap
// and it doesn't move the data ownership from the original variable



// and yeah, a reference is just a pointer!

// borrowing creates another pointer for the same data in the heap
// so that when it's function frame is released, the data in heap won't move

// ownership (no-borrow) moves the data from the heap to another address
// then it's new pointer is stored in the variable that took ownership