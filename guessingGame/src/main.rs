use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guesse the number");

    let secret_number = rand::tread_rng().gen_range(1..=100);
    
    loop{
        println!("Please input your guess");
    }
}