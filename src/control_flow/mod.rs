
pub fn run() {
    // if, else if, else
    let x = 100;
    if x > 100 || x < 100 && x == 20 {
        println!("impossible");
    }
    else if x == 20 {
        println!("should not print this");
    }
    else {
        println!("if is anwsome");
    }

    // while loop
    let mut count = 0;
    while count < 3 {
        println!("while loop count {}", count);
        count += 1;
    }

    // loop forever
    count = 0;
    'loop_name: loop {
        println!("loop count {}", count);
        let mut inner_loop_count = 0;

        loop {
            if inner_loop_count > 2 {
                break;
            }
            if count > 2 {
                println!("break loop by name");
                break 'loop_name;
            }

            inner_loop_count += 1;
        }
        count += 1;
    }

    // for in
    let a = [1,2,3];
    for i in a.iter() {
        println!("for in {}", i);
    }
}