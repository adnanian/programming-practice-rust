fn main() {
    // IF ELSE STATEMENTS
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // TERNARY EXPRESSIONS
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // LOOPS
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }

    // WHILE
    let mut number = 3;

    while number != 0 {
        println!("WHILE {number}!");

        number -= 1;
    }

    println!("WHILE LIFTOFF!!!");

    // FOR
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("FOR: the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("FOR REV: {number}!");
    }
    println!("FOR LIFTOFF!!!");
}
