fn my_num() {
    let my_numbers: i16 = 1_0________________________________;
    println!("{}", my_numbers); // 10
    {
        println!("{}", my_numbers); // 10
        let my_numbers: i16 = 20;
        println!("{}", my_numbers); // 20
    }
    println!("{}", my_numbers); // 10

    println!("test");
}

fn main() {
    my_num();
}
