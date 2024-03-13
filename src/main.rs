use rand::Rng;
use std::io::stdin;
fn main() {
    //take input
    println!("Please type a number from 1 to 3:");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input: usize = input.trim().parse().unwrap();
    let mut selected_door = [false; 3];
    selected_door[input - 1] = true;

    //selecting winning door
    let winner: usize = rand::thread_rng().gen_range(0..=2);
    let mut winning_door = [false; 3];
    winning_door[winner] = true;

    //remove a door that is niether winning or selected
    loop {
        let random = rand::thread_rng().gen_range(0..=2);
        println!("{random}");
        if winning_door[random] && selected_door[random - 1] == false {
            let removed_door = random;
            println!("{}", removed_door);
            break;
        }
    }
}
