use binance::api::*;
use binance::market::*;
fn main() {
    let market: Market = Binance::new(None, None);
    match market.get_price("BTCUSDT") {
        Ok(answer) => {
            let price = answer.price;
            println!("{:?}", price)
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
