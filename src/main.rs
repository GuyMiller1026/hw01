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
fn main() {
}
