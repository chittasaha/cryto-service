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
use crate::models::{Currency, Token};
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

    
    let mut tokens:Vec<Token> = vec![]; 
    
    let coint_list = client.coins_list(false)
    .await.unwrap();

    for t in coint_list
    {
        
        if t.id == "" 
        {
            continue;
        }
        
        let coin = match client.coin(&t.id, false,false,true,true,true,true).await
        {
            Ok(c) => c,
            Err(_e) => continue,
        };
        
        
        
        let id = coin.id;
        let name = coin.name;

        let market_data = coin.market_data.unwrap();
        let maket_cap = market_data.market_cap.eur.unwrap();

        let devs_data = coin.developer_data.unwrap();
        let total_devs = devs_data.pull_request_contributors.unwrap();
        let last4_weeks_commit = devs_data.commit_count4_weeks.unwrap();

        if maket_cap < 2000000.0 || total_devs < 2.0
        {
            continue;
        }

        let all_time_high = market_data.ath.eur.unwrap();
        let ath_date =market_data.ath_date.usd.unwrap();
        let all_time_low = market_data.atl.eur.unwrap();
        let atl_date = market_data.atl_date.usd.unwrap();
        let current_price = market_data.current_price.eur.unwrap();
        let ath_change_parcent = market_data.ath_change_percentage.usd.unwrap();

        //let circulating_supply = coin.market_data.unwrap().circulating_supply;
        //let max_supply = market_data.max_supply;

        let catagories = coin.categories.iter().map(|c| c.to_string() + ",").collect::<String>();
        let description = coin.description.en.unwrap();
        let twitter_follower = coin.community_data.unwrap().twitter_followers.unwrap();
        //let market_rank = String::from(coin.market_cap_rank.as_str().unwrap());

        //println!("{:?}", coin.clone());

        let token = Token{
            id : id,
            name : name,
            catagories : catagories,
            description : description,
            market_cap: maket_cap,
            market_rank : String::from(""),
            all_time_high : all_time_high,
            ath_date : ath_date,
            all_time_low : all_time_low,
            atl_date : atl_date,
            ath_change_parcent : ath_change_parcent,
            current_price : current_price,
            //cir_supbn/ ly : circulating_supply,
            //max_supply }: max_supply,
            total_devs : total_devs,
            last4_weeks_commit : last4_weeks_commit,
            twitter_followers : twitter_follower,            
        };

        //println!("{:?}", token.clone());
        
        tokens.push(token);

    }

    

    //coin.developer_data.unwrap().
    
    Ok(Json(tokens))
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
            let mut change_24:f64 = 0.0;
            let mut market_capital:f64 = 0.0; 
            let name = String::from(cur);
            if prices.contains_key(cur)
            {
                //println!("{0}",cur);
                price = prices[cur].eur.unwrap();
                change_24 = prices[cur].eur24_h_change.unwrap();
                market_capital = prices[cur].eur_market_cap.unwrap();
                //info!("{:?}", prices[cur]);
            }

            

            response_price.push(Currency
                {
                    base_currency: base_cur,
                    price: price,
                    name : name,
                    change_last_24_hours: Some(change_24),
                    market_capital: Some(market_capital)
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
