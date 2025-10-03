
fn main() {
    println!("📚 Crypto Glossary");
    println!("Istilah crypto untuk pemula");
}
mod commands;

fn main() {
    println!("📚 Crypto Glossary CLI");
    commands::check_price();
}
mod commands;
mod data;

fn main() {
    println!("📚 Crypto CLI Tool");
    commands::check_price();
    commands::list_coins();
    println!("\n🔍 Info Coin:");
    println!("{}", data::get_coin_info("BTC"));
}
