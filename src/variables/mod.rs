pub fn run() {
    // default is int32
    let immutable_var = 1;
    
    // var assign types
    let type_var: u8 = 5;

    // mutable_var
    let mut mutable_var = 2;
    mutable_var = 3;

    let a_boolean: bool = true;
    let character: char = 't';

    // string
    let mut string_testing = String::from("hello");
    let new_string: &str = "test";
    let s = "testing".to_string();

    string_testing.push_str(new_string);

    println!("x + y = {}", immutable_var + mutable_var);
    println!("string = {}", string_testing);
}