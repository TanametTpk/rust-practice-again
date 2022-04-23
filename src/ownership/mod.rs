fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} 
// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn takes_reference(some_string: &String) {
    // some_string comes into scope
    println!("{}", some_string);
} 
// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} 

pub fn run() {
    let s1 = String::from("test");
    let s2 = s1;
    // now s1 this invalid, can't not use anymore!!

    let s1 = String:: from("new string");
    let s3 = &s1;
    // s3 this borrowing s1 via refernce

    println!("s3 borrowing s1 = {}", s3);

    // be careful
    let a = 32;
    makes_copy(a);
    println!("a still can use {}", a);

    takes_reference(&s1);
    // s1 still can use
    println!("func take ref = {}", s1);

    takes_ownership(s1);
    // s1 can not use
}