use std::io;

fn main() {
    let mut side_deck = String::new();
    println!("Enter your side deck in a space separated list\nInvalid entries will result in four random cards");
    io::stdin()
        .read_line(&mut side_deck)
        .ok()
        .expect("No side deck entered");
    let side_deck: Vec<u32> = side_deck
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    println!("Your side deck is: ");
    for card in side_deck {
        println!("{}", card)
    }
    println!("Let's play Pazaak!");
    // let mut player_a_score = 0;
    // let mut player_b_score = 0;
    // loop {
    //     if player_a_score = 20 || if player_b_score = 20 { }
    // }
}
