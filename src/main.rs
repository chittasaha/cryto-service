use coingecko_rs::{CoinGeckoClient};
use serde::__private::de::IdentifierDeserializer;
use serde_json::from_str;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use crate::models::Currency;
use tracing::{info, Level};

pub mod models;

#[tokio::main]
async fn main() {
    // initialize tracing
    //tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /prices` goes to `get_prices`
        .route("/prices", get(get_prices))
        .route("/get_tokens", get(get_tokens));
        // `POST /users` goes to `create_user`
        //.route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_tokens() -> Result<impl IntoResponse, StatusCode> {
    
    let client = CoinGeckoClient::default();
    let result = client.coins_list(true)
    .await.unwrap();
    
    Ok(Json(result))
}


async fn get_prices() -> Result<impl IntoResponse, StatusCode> {

    
           
        let vec_currency = vec!["acala","cardano","altair","altura","astar","avalanche-2","centrifuge","coti","curve-dao-token","polkadot","efinity","ethereum","fantom","moonbeam","kilt-protocol","kintsugi","calamari-network","kusama","decentraland","moonriver","harmony","parallel","the-sandbox","shiden"];
        
        //let currencies = vec_currency.iter().map(|x| x.to_string() + ",").collect::<String>();
        
        let client = CoinGeckoClient::default();            

        let prices= match client.price( &vec_currency,
             &vec!["eur"],true, true,
              true, false).await
        {
            Ok(p) => p,
            Err(_error) => panic!(""),
        };

        let mut response_price: Vec<Currency> = Vec::new();

        for cur in vec_currency
        {
            let base_cur = String::from("eur");
            let mut price:f64= 0.0;
            let name = String::from(cur);
            if prices.contains_key(cur)
            {
                //println!("{0}",cur);
                price = prices[cur].eur.unwrap();
            }

            response_price.push(Currency
                {
                    base_currency: base_cur,
                    price: price,
                    name : name,
                    change_last_24_hours: None,
                    market_capital: None
                });
            
            
        } 

        

        Ok(Json(response_price))

}

// fn test()
// {
//     let k =22;
//     let x : Result<_, &str> = Err("foo");
//     let res1  = x.map_or_else(|e| "k * 2", |v| v);

//     let x : Result<&str, _> = Err("bar");
//     let res2 = x.map_or_else(|e| k * 2, |v| v.len());

//     println!("Res1 = {:#?}, and Res2={:#?}", res1, res2);

//     //Ok(());
// }
