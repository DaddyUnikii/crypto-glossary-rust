
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
mod commands;
mod data;
mod ui;

fn main() {
    println!("📚 Crypto CLI Tool");
    ui::show_menu();
    // Simulasi input pengguna
    let choice = "1"; // Simulasi pilihan 1
    ui::handle_choice(choice);
}
// Import modul tests (simulasi)
#[cfg(test)]
mod tests;

mod commands;
mod data;
mod ui;
mod api;

fn main() {
    println!("📚 Crypto CLI Tool");
    ui::show_menu();
    // Simulasi input pengguna
    let choice = "1"; // Simulasi pilihan 1
    ui::handle_choice(choice);

    // Simulasi API call
    println!("\n🔍 Cek harga Bitcoin via API:");
    println!("{}", api::get_crypto_price("BTC"));
    println!("\n📈 Chart Bitcoin 7 hari:");
    println!("{}", api::get_crypto_chart("BTC"));
}
