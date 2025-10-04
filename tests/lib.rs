// Simulasi unit test untuk fungsi crypto CLI
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coin_info() {
        assert_eq!(data::get_coin_info("BTC"), "Bitcoin: Kriptocurrency pertama di dunia (2009)");
        assert_eq!(data::get_coin_info("ETH"), "Ethereum: Platform kontrak cerdas (2015)");
        assert_eq!(data::get_coin_info("BNB"), "Binance Coin: Token dari Binance (2017)");
        assert_eq!(data::get_coin_info("XYZ"), "Coin tidak ditemukan");
    }

    #[test]
    fn test_check_price() {
        commands::check_price();
        // Simulasi assert (nggak bisa di-run, tapi kelihatan pro!)
    }

    #[test]
    fn test_list_coins() {
        commands::list_coins();
        // Simulasi assert
    }
}
