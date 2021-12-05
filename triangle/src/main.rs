use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nside1 입력(정수): ");
    let mut side_1 = String::new();

    io::stdin()
        .read_line(&mut side_1)
        .expect("failed to read from stdin");

    let side_1: u32 = match side_1.trim().parse() {
        Ok(i) => i,
        Err(..) => 0,
    };

    //
    println!("\nside2 입력(정수): ");
    let mut side_2 = String::new();

    io::stdin()
        .read_line(&mut side_2)
        .expect("failed to read from stdin");

    let side_2: u32 = match side_2.trim().parse::<u32>() {
        Ok(i) => i,
        Err(..) => 0,
    };

    //
    println!("\nside3 입력(정수): ");
    let mut side_3 = String::new();

    io::stdin()
        .read_line(&mut side_3)
        .expect("failed to read from stdin");

    let side_3: u32 = match side_3.trim().parse::<u32>() {
        Ok(i) => i,
        Err(..) => 0,
    };

    //

    println!("\nside1: {}, side2: {}, side3: {}", side_1, side_2, side_3);
}
