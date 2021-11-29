
fn my_num() {
    let my_numbers = 10;
    println!("{}", my_numbers);
    {
        let my_numbers = 20;
        println!("{}",my_numbers);
    }
    println!("{}", my_numbers);
}

fn main() {
    my_num();
}