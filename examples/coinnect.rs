extern crate coinnect;

use coinnect::coinnect::Coinnect;
use coinnect::gdax::GdaxCreds;
use coinnect::exchange::Exchange::Gdax;
use coinnect::types::Pair::BTC_USD;

fn main() {
    let creds = GdaxCreds::new("", "", "", "");
    let mut api = Coinnect::new(Gdax, creds).unwrap();
    let ticker = api.ticker(BTC_USD);

    println!("BTC_USD last trade price is: {}", ticker.unwrap().last_trade_price);
}
