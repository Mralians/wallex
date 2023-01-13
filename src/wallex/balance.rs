#![allow(non_snake_case)]

use serde::Deserialize;
use std::collections::HashMap;

use super::error::{err_non_ok_response,WallexError};
use super::response::WallexResp;

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResult {
    pub balances: HashMap<String, Balance>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    pub locked: String,
    pub value: String,
    pub fiat: bool,
    pub faName: String,
    pub asset: String,
}
const BASE_URL: &'static str = "https://api.wallex.ir";
impl BalanceResult {
    pub fn new(api_key: &str) -> Result<Self, WallexError> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(BASE_URL.to_string() + "/v1/account/balances")
            .timeout(std::time::Duration::from_secs(5))
            .header("X-API-Key", api_key)
            .header("Content-Type", "application/json")
            .send()?
            .json::<WallexResp<BalanceResult>>()?;
        let status_code = reqwest::StatusCode::from_u16(resp.status_code())?;
        if status_code != reqwest::StatusCode::OK {
            err_non_ok_response(status_code)?
        }
        let balance = resp.result().clone().unwrap();
        Ok(balance)
    }
    pub fn get_asset_value(&self, currency: &str) -> f64 {
        let value = self.balances[currency].value.trim().parse::<f64>().unwrap();
        value
    }
    pub fn balances() {
        todo!()
    }
}
