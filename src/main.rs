use std::io;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;
const TRADE_DICE: u64 = 3;
const TRADE_MULTIPLIER: u64 = 2;
fn print_menu() {
    println!();
    println!("1) Buy food");
    println!("2) Hunt");
    println!("3) Trade with caravan");
    println!("4) View status");
    println!("5) Quit");
}
fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}
fn hunt_food(roll: u64) -> u64 {
    if roll == 1 {
        0
    } else if roll <= 3 {
        10
    } else if roll <= 5 {
        20
    } else {
        40
    }
}
fn read_number(min: u64, max: u64) -> u64 {
    loop {
        println!("Enter a number from {} to {}:", min, max);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: u64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("That is not a whole number. Try again.");
                continue;
            }
        };

        if number < min || number > max {
            println!("That number is out of range. Try again.");
            continue;
        }

        return number;
    }
}
fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;
    for i in 0..count {
        total += roll_die(seed + i);
    }
    total
}
fn print_status(gold: u64, food: u64, day: u64) {
    println!("Day: {}", day);
    println!("Gold: {}", gold);
    println!("Food: {}", food);
}
fn main() {
    println!("Welcome to Trail Trader!");
    println!("Pick a seed.");
    let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();
        let choice = read_number(1, 5);

        if choice == 1 {
            if can_afford(gold, FOOD_PRICE) {
                println!("How many units of food?");
                let amount = read_number(1, gold / FOOD_PRICE);
                gold -= amount * FOOD_PRICE;
                food += amount;
                day += 1;
            } else {
                println!("You cannot afford any food.");
            }
        } else if choice == 2 {
            let roll = roll_die(seed + roll_count);
            roll_count += 1;
            food += hunt_food(roll);
            day += 1;
            println!("You rolled a {}.", roll);
        } else if choice == 3 {
            let total = roll_many(seed + roll_count, TRADE_DICE);
            roll_count += TRADE_DICE;
            gold += TRADE_MULTIPLIER * total;
            day += 1;
            println!("The caravan rolled a total of {}.", total);
        } else if choice == 4 {
            print_status(gold, food, day);
        } else {
            println!("Final status:");
            print_status(gold, food, day);
            break;
        }
    }
}
