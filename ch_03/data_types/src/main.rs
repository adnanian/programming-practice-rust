fn main() {
    let num1 = 98222;
    let num2 = 98_222;

    println!("{}", num1);
    println!("{}", num2);

    let tup = (500, 6.4, 1);

    // Destructuring elements.
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // How to modify elements in a MUTABLE tuple.
    let mut x2: (i32, i32) = (1, 2);
    x2.0 = 0;
    x2.1 += 5;

}
