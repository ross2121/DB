use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use schema::{trades, orders};
use std::env;
use redis::{Client, Commands};
use serde::{Deserialize, Serialize};
use tokio;
use serde_json;
use validator::Validate;
use dotenv::dotenv;
use diesel::prelude::*;


use crate::model::{DbMessage, OrderUpdateData, TradeData};

mod model;
mod schema;



pub type dbpool=r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn main_connection()->dbpool{
  match dotenvy::dotenv() {
      Ok(_)=>print!("db file found"),
      Err(e)=>println!("could not find env")
  }
  let database= match env::var(" DATABASE_URL") {
      Ok(url)=>url,
      Err(e)=>panic!("{}", e),
  };
  let dbconnection=ConnectionManager::<PgConnection>::new(database);
  r2d2::Pool::builder().build(dbconnection).expect("Failed to connect")
}
pub async fn db_processo(pool:dbpool){
   let client=Client::open("http").expect("Errror connecting");
   let mut conn=client.get_connection().expect("Error connectig");
   loop {
    let result:Option<String> = conn.rpop("db_processor", Some(std::num::NonZero::new(1).unwrap())).expect("Not able to pop msg");
     if let Some(message)=result{
      match serde_json::from_str::<DbMessage>(&message) {
          Ok(message)=>{
            match process_message(message, &pool) {
                Ok(_)=>print!("Suceefull added"),
                Err(e)=>print!("Error processing message")
            }
          }
          Err(e)=>print!("Error debugign message")
      }
     }
   }
}
pub fn process_message(message: DbMessage,pool:&dbpool)->Result<(),diesel::result::Error>{
    let conn = &mut pool.get().unwrap();
      match  message {
          DbMessage::TradeAdded(tradmessage)=>{
            let trade=TradeData{
                id:tradmessage.id.to_string(),
                is_buyer_marker:tradmessage.is_buyer_marker,
                market:tradmessage.market,
                price:tradmessage.price,
                quantity:tradmessage.quantity,
                quote_quantity:tradmessage.quote_quantity
            };
            diesel::insert_into(trades::table).values(trade).execute(conn)?;
          }
          DbMessage::OrderUpdate(orderupdate)=>{
            let order=OrderUpdateData{
                executed_qty:orderupdate.executed_qty,
                order_id:orderupdate.order_id,
                market:orderupdate.market,
                price:orderupdate.price,
                quantity:orderupdate.quantity,
                side:orderupdate.side
            };
            diesel::insert_into(orders::table).values(order).execute(conn)?;
          }
      }
    Ok(())
}
#[tokio::main]
pub async fn main(){
  dotenv().ok();
  env_logger::init();
  
  println!("Starting DB processor...");

  let pool =main_connection();
  println!("Successfully connected to database");
  db_processo(pool).await;
}
