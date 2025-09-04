
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema::{orders, trades};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = orders)]
pub struct OrderUpdateData {
  pub  order_id: String,
    pub executed_qty: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
}
#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = trades)]
pub struct TradeData {
    pub id: String,
    pub is_buyer_marker: bool,
    pub price: i64,
    pub quantity:i64,
    pub quote_quantity: String,
    pub market: String,
}
#[derive(Serialize,Deserialize,Clone,PartialEq,Debug)]
pub enum Side  {
    Sell,Buy
}