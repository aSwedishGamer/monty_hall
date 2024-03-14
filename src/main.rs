use rand::Rng;
use std::io::stdin;

fn win_loose(arr: [bool; 3], arr1: [bool; 3]) {
    if arr.iter().position(|&x| x == true) == arr1.iter().position(|&x| x == true) {
        println!("You win!!")
    } else {
        println!("You loose")
    }
}

fn main() {
    // Take input
    println!("Please type a number from 1 to 3:");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let mut input: usize = input.trim().parse().unwrap();
    let mut selected_door = [false; 3];
    input -= 1;
    selected_door[input] = true;

    // Selecting winning door
    let winner: usize = rand::thread_rng().gen_range(0..=2);
    let mut winning_door = [false; 3];
    winning_door[winner] = true;

    // Remove a door that is neither winning or selected
    let mut removed_door = [false; 3];
    loop {
        let random = rand::thread_rng().gen_range(0..=2);
        if winning_door[random] == false && selected_door[random] == false {
            removed_door[random] = true;
            println!(
                "If we remove door {} do you still want to keep door number {}? Y or N",
                random + 1,
                input + 1
            );
            break;
        }
    }

    // Take new input for changing door
    let mut y_n = String::new();
    let _ = stdin().read_line(&mut y_n);
    let y_n: &str = y_n.trim();
    if y_n == "n" || y_n == "N" {
        loop {
            let random = rand::thread_rng().gen_range(0..=2);
            if selected_door[random] == false && removed_door[random] == false {
                let mut selected_door = [false; 3];
                selected_door[random] = true;

                // Decide if user wins or not
                win_loose(selected_door, winning_door);
                break;
            }
        }
    } else {
        win_loose(selected_door, winning_door);
    }
}
