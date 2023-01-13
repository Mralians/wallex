use dotenv::dotenv;

mod wallex;

fn main() {
    dotenv().ok();
    //let api_key = dotenv::var("API_KEY").unwrap();
    let market = wallex::market::MarketResult::new().unwrap();
    println!("{:#?}",market.symbol["ETCTMN"]);
}
