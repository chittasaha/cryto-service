use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Currency
{
    pub name: String,
    pub price: f64,
    pub base_currency: String,
    pub change_last_24_hours : Option<f32>,
    pub market_capital: Option<f64>
}