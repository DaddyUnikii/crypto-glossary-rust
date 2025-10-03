// Data simulasi crypto
pub fn get_coin_info(coin: &str) -> String {
    match coin {
        "BTC" => "Bitcoin: Kriptocurrency pertama di dunia (2009)".to_string(),
        "ETH" => "Ethereum: Platform kontrak cerdas (2015)".to_string(),
        "BNB" => "Binance Coin: Token dari Binance (2017)".to_string(),
        _ => "Coin tidak ditemukan".to_string(),
    }
}
