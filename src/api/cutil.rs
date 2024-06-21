use std::error::Error;
use std::time::Duration;
use reqwest::{Client as async_client, StatusCode};

use crate::cfg::RespItem;

pub async fn create_client() -> async_client {
    return reqwest::Client::builder()
        .timeout(Duration::from_secs(3)).build().unwrap()
}

pub async fn exec_url_as(url: String) -> RespItem {
    let client = super::cutil::create_client().await;
    let mut item = RespItem{site: "".to_string(), code: 200};

    let resp = client
        .get(format!("{}", url))
        .send()
        .await;

    if resp.is_ok() {
        let r = resp.as_ref();
        if r.unwrap().status() != StatusCode::OK {
            item = RespItem{code: r.unwrap()
                .status()
                .as_str()
                .parse::<i32>()
                .unwrap(), site: format!("{}", url).to_string() };
                // .unwrap(), site: url };
        } else {
            println!("REQ :: {}", url);
        }
        
    } else if resp.is_err() {
        let rr = resp.as_ref();
        // println!("ERR RESP :: {:#?}", rr.err().unwrap());
        println!("ERR RESP :: {:#?}", rr.err().unwrap().source().unwrap());
        // item = RespItem{code: rr.unwrap()
        //     .status()
        //     .as_str()
        //     .parse::<i32>()
        //     .unwrap(), site: url.to_string()};
    }
    
    return item;
}
