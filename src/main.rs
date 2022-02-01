use coingecko::{Client, SimplePriceReq};


pub fn main() {
    smol::block_on(async {
        
        let vec_currency = vec!["acala","cardano","altair","altura","astar","avalanche-2","centrifuge","coti","curve-dao-token","polkadot","efinity","ethereum","fantom","moonbeam","kilt-protocol","kintsugi","calamari-network","kusama","decentraland","moonriver","harmony","parallel","the-sandbox","shiden","stratos"];
        
        
        let currencies = vec_currency.iter().map(|x| x.to_string() + ",").collect::<String>();
        
        
        
        let http = isahc::HttpClient::new().unwrap();

        let client = Client::new(http);

        //let coin_list = client.coins_list().await.ok().unwrap();
        
        // for c in coin_list
        // {
        //     println!("{0},{1},{2}",c.id,c.symbol,c.name);
        // };
        let req = SimplePriceReq::new(currencies.into(), "eur".into())
            .include_market_cap()
            .include_24hr_vol()
            .include_24hr_change()
            .include_last_updated_at();
            

        let prices= match client.simple_price(req).await
        {
            Ok(p) => p,
            Err(_error) => panic!(""),
        };

        for cur in vec_currency
        {
            if prices.contains_key(cur)
            {
                //println!("{0}",cur);
                println!("{0}",prices[cur]["eur"]);
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
    })
}
