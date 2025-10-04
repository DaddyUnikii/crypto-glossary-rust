
# Crypto Glossary (Rust) 📚

Kamus istilah crypto untuk pemula.  
*Project ini 100% di-browser—nggak perlu install Rust!* 😎

## 📦 Struktur Project
- `Cargo.toml`: Konfigurasi project
- `src/main.rs`: File utama CLI
- `src/lib.rs`: File library (kosong untuk sekarang)
- `.gitignore`: File yang diabaikan oleh Git
- `LICENSE`: Lisensi MIT

## 🧪 Fungsi CLI
- `check_price()`: Simulasi cek harga Bitcoin.
- `list_coins()`: Daftar coin populer.
- `get_coin_info(coin)`: Info singkat tentang coin.

## 📦 Data Simulasi
- File `data.rs` menyimpan info coin (BTC, ETH, BNB).

## 📋 Menu Interaktif
- `show_menu()`: Tampilkan menu pilihan.
- `handle_choice(choice)`: Proses pilihan pengguna (simulasi).

## 🧪 Contoh Penggunaan
```bash
$ cargo run
🚀 Crypto CLI Menu
1. Cek Harga Bitcoin
2. Daftar Coin
3. Info Coin
4. Keluar
Pilih opsi (1-4): 1
💰 Cek harga Bitcoin...


## 🚀 Catatan
- Project ini nggak bisa di-run karena nggak ada Rust di komputer lo.
