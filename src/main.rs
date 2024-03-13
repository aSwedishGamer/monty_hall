use rand::Rng;
use std::{array, io::stdin};
fn main() {
    let winner: usize = rand::thread_rng().gen_range(0..=3);
    println!("Please type a number from 1 to 3:");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input: i8 = input.trim().parse().unwrap();
    let mut array: [i8; 3] = [0; 3];
    if input == winner.try_into().unwrap() {
        array[winner - 1] = 2;
    } else {
        array[winner - 1] = 1;
    }
}
