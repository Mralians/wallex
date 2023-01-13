use chrono::{DateTime, Utc};

use http::StatusCode;
use serde::{de, Deserialize};
use std::collections::HashMap;

use super::error::{err_non_ok_response, WallexError};
use super::response::WallexResp;

#[derive(Debug, Clone, Deserialize)]
pub struct MarketResult {
    #[serde(rename = "symbols")]
    pub symbol: HashMap<String, Market>,
}

// market information

#[derive(Debug, Clone, Deserialize)]
pub struct Market {
    pub symbol: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: u32,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: u32,
    #[serde(rename = "faName")]
    pub fa_name: String,
    #[serde(rename = "faBaseAsset")]
    pub fa_base_asset: String,
    #[serde(rename = "faQuoteAsset")]
    pub fa_quote_asset: String,
    #[serde(rename = "stepSize")]
    pub step_size: u32,
    #[serde(rename = "tickSize")]
    pub tick_size: u32,
    #[serde(rename = "minQty")]
    pub min_qty: f64,
    #[serde(rename = "minNotional")]
    pub min_notional: f64,
    pub stats: Stats,

    #[serde(deserialize_with = "deserialize_timestamp")]
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stats {
    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "askPrice")]
    pub ask_price: Option<f64>,

    #[serde(rename = "24h_ch")]
    pub change_24h: f64,

    #[serde(rename = "7d_ch")]
    pub change_7d: f64,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "24h_volume")]
    pub volume_24h: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "7d_volume")]
    pub volume_7d: Option<f64>,

    //pub	QuoteVolume24H,
    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "24h_highPrice")]
    pub high_price_24h: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "24h_lowPrice")]
    pub low_price_24h: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "lastPrice")]
    pub last_price: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "lastQty")]
    pub last_qty: Option<f64>,

    #[serde(rename = "lastTradeSide")]
    pub last_trade_side: String,

    #[serde(deserialize_with = "deserialize_f64")]
    #[serde(rename = "bidVolume")]
    pub bid_volume: Option<f64>,

    //#[serde(deserialize_with = "deserialize_f64")]
    //#[serde(rename = "askVolume")]
    //pub	ask_volume:Option<f64>,

    //#[serde(deserialize_with = "deserialize_u32")]
    //#[serde(rename = "bidCount")]
    //pub	bid_count:Option<u32>,
    pub direction: Direction,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Direction {
    #[serde(rename = "SELL")]
    pub sell: f64,
    #[serde(rename = "BUY")]
    pub buy: f64,
}

const BASE_URL: &'static str = "https://api.wallex.ir";
impl MarketResult {
    pub fn new() -> Result<Self, WallexError> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(BASE_URL.to_string() + "/v1/markets")
            .timeout(std::time::Duration::from_secs(5))
            .header("Content-Type", "application/json")
            .send()?
            .json::<WallexResp<MarketResult>>()?;
        let status_code = StatusCode::from_u16(resp.status_code())?;
        if status_code != reqwest::StatusCode::OK {
            err_non_ok_response(status_code)?
        }
        let market = resp.result().clone().unwrap();
        Ok(market)
    }
}

fn deserialize_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    if s == "-" {
        return Ok(None);
    }
    let value = s.trim().parse::<f64>().map_err(de::Error::custom)?;
    Ok(Some(value))
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let date_str: &str = de::Deserialize::deserialize(deserializer)?;
    let datetime = DateTime::parse_from_rfc3339(date_str).map_err(de::Error::custom)?;
    let datetime_utc = datetime.with_timezone(&Utc);
    Ok(datetime_utc)
}


