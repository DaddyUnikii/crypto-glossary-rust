// Simulasi API crypto (nggak bisa di-run, tapi kelihatan pro!)
pub fn get_crypto_price(coin: &str) -> String {
    match coin {
        "BTC" => "Bitcoin: $60,000 (simulasi)".to_string(),
        "ETH" => "Ethereum: $3,000 (simulasi)".to_string(),
        "BNB" => "Binance Coin: $500 (simulasi)".to_string(),
        _ => "Coin tidak ditemukan".to_string(),
    }
}

pub fn get_crypto_chart(coin: &str) -> String {
    match coin {
        "BTC" => "📈 BTC: $55k → $60k → $65k (7 hari)".to_string(),
        "ETH" => "📈 ETH: $2.8k → $3k → $3.2k (7 hari)".to_string(),
        _ => "Data chart tidak tersedia".to_string(),
    }
}
