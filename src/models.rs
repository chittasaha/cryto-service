use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency
{
    pub name: String,
    pub price: f64,
    pub base_currency: String,
    pub change_last_24_hours : Option<f64>,
    pub market_capital: Option<f64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token
{
    pub id : String,
    pub name: String,
    pub catagories : String,
    pub description: String,
    pub market_cap : f64,
    pub twitter_followers : f64,
    pub market_rank: String,
    pub all_time_high : f64,
    pub ath_date : String,
    pub all_time_low : f64,
    pub atl_date : String,
    pub ath_change_parcent : f64,
    pub current_price : f64,
    //pub cir_supply : Value,
    //pub max_supply : Value,
    pub total_devs :f64,
    pub last4_weeks_commit : f64


}