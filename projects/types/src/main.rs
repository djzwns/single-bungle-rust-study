use std::io;

fn main() {
    /* types */
    //let guess: u32 = "42".parse().expect("Not a number!");
    //let literals_1 = 10_100u8; // out of range !! 
    //println!("literal1: {}", literals_1);
    //let x = 2.0;
    //let y: f32 = 3.0;
    //let a = 10;
    //let b: i64 = 20;

    /* numeric operations */
    //let sum = 5 + 10;
    //let difference = 95.5 - 4.3;
    //let product = 4 * 30;
    //let quotient = 56.7 / 32.2;
    //let floored = 2 / 3;
    //let remainder = 43 % 5;

    //println!("sum: {}, sub: {}", sum, difference);
    //println!("mul: {}, div1: {}, div2: {}", product, quotient, floored);
    //println!("remainder: {}", remainder);

    /* tuple */
    //let tup = (500, 6.4, 1);
    //let (x, y, z) = tup;
    //println!("The value of x is: {}", x);

    /* arrays */
    /*
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    a[0] = 10;
    println!("first value: {}, second value: {}", first, second);
    */

    /* invalid array element access */
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

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
