// Simulasi menu interaktif
pub fn show_menu() {
    println!("🚀 Crypto CLI Menu");
    println!("1. Cek Harga Bitcoin");
    println!("2. Daftar Coin");
    println!("3. Info Coin");
    println!("4. Keluar");
    println!("Pilih opsi (1-4): ");
}

pub fn handle_choice(choice: &str) {
    match choice {
        "1" => println!("💰 Cek harga Bitcoin..."),
        "2" => println!("📋 Daftar Coin: BTC, ETH, BNB"),
        "3" => println!("🔍 Info Coin: Masukkan simbol coin"),
        "4" => println!("👋 Keluar..."),
        _ => println!("❌ Pilihan tidak valid"),
    }
}
