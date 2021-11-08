mod main1;
mod main2;
mod main3;
mod main4;
mod main5;
mod main6;


fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
    println!("Hello, world!");

    main1::main1();
    println!(" ");
    main2::main2();
    println!(" ");
    main3::main3();
    println!(" ");
    main4::main4();
    // println!(" ");
    // main5::main5();
    println!(" ");
    main6::main6();
}