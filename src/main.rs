//use coingecko::{Client, SimplePriceReq};

use coingecko_rs::{CoinGeckoClient};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
        
        let vec_currency = vec!["acala","cardano","altair","altura","astar","avalanche-2","centrifuge","coti","curve-dao-token","polkadot","efinity","ethereum","fantom","moonbeam","kilt-protocol","kintsugi","calamari-network","kusama","decentraland","moonriver","harmony","parallel","the-sandbox","shiden"];
        
        //let currencies = vec_currency.iter().map(|x| x.to_string() + ",").collect::<String>();
        
        let client = CoinGeckoClient::default();            

        let prices= match client.price( vec_currency.clone(), vec!["eur"],true, true, true, false).await
        {
            Ok(p) => p,
            Err(_error) => panic!(""),
        };

        for cur in vec_currency
        {
            if prices.contains_key(cur)
            {
                //println!("{0}",cur);
                println!("{:?}",prices[cur].eur.unwrap());
            }
            else
            {
                println!("{0}",0);
            }
            
        }

        // for cur in currencies.clone().split(',')
        // {
        //     println!("{0}", cur);
        // }

        // let something = curs.for_each(|str | println!("{0}", str));

        //let mut curs = currencies.split(',').into_iter();

        //let test = curs.for_each( |cur| println!("{0}",prices[cur]["eur_24h_vol"]));

        
        // for(key, value) in &prices{
        //     println!("{0}           {1}         {2}", key, value["eur"], value["eur_24h_vol"]);
        // }

        //println!("{:#?}", prices);
        
        
            //println!("{:#?}", );
        //println!("{:#?}", client.coins_list().await);
        //println!("{:#?}", client.coin_info(&"cardano").await);

        Ok(())

}
