fn add(x: i32) -> i32 {
    x + 1
}

fn test(x: i32) -> i32 {
    if x > 100 {
        return x;
    }
    return x + 1;
}

pub fn run() {
    println!("func add = {}", add(3));
    println!("func test = {}", test(101));
}