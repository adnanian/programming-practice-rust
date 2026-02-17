fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
    and_another_function(5);
}

// Arguments/parameters must ALWAYS have their types declared.
fn and_another_function(x: i32) {
    println!("The value of x is: {x}");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
