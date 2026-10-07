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
fn main() {
}
