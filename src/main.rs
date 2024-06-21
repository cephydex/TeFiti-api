mod cfg;
mod api;
mod lib;
use api::routes;
use warp::Filter;
use dotenv::dotenv;
// use lib;


#[tokio::main]
async fn main() {
    open_data_file().await;
    dotenv().ok();

    let index2 = warp::get()
        .and(warp::path("v1"))
        .and(warp::path::end())
        .and_then(routes::index_page);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("hello {}", name));

    let index1 = warp::get()
        .and(warp::path::end())
        .and_then(routes::index_page);

    let download_readme = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("files"))
        .and(warp::path("readme.md"))
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    let download_monitor = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("files"))
        .and(warp::path("monitor.txt"))
        .and(warp::path::end())
        .and(warp::fs::file("./monitoring.txt"));

    let routes = hello
        .or(download_readme)
        .or(download_monitor)
        .or(index1)
        .or(index2)
        .with(warp::cors().allow_any_origin());

    tokio::spawn({
        let api_port = env::var("API_PORT").expect("App API could not be retrieved.");
        warp::serve(routes)
            .run(([0, 0, 0, 0], api_port.parse().unwrap()))
            // .run(([127, 0, 0, 1], api_port.parse().unwrap()))
            // .run(([0, 0, 0, 0], 3003))
    });

    // // let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(20).to_std().unwrap());
    let mut interval_timer = tokio::time::interval(chrono::Duration::minutes(7).to_std().unwrap());
    loop {
        interval_timer.tick().await;
        tokio::spawn(async { run_bundle().await; }); // async task
    }

}

use std::{env, fs::{File, OpenOptions}, io::Write};

use tokio::task::JoinSet;

use crate::{api::cutil::exec_url_as, cfg::{print_date, RespItem}};

pub async fn run_bundle() {
    let (dt_str, _dt2_str) = print_date();
    let tt: Vec<RespItem> = lookup_concur().await;
        
    append_respitem_to_file(&tt, &dt_str).await;

    // if result length > 0 then send message to admin
    println!("RES >> {:?}", tt);
    if tt.len() > 0 {
        let content_str = proc_data_msg(&tt);
        let msg_string: String = format!("{} | {}", &dt_str, content_str);

        // send SMS to admins
        // lib::send_message(&msg_string).await;

        // get admin emails and send emails
        let email_list = lib::get_list_env("ADMIN_EMAILS");
        let emails: Vec<&str> = email_list.iter().map(|n| n.as_str()).collect();
        println!("Admin emails :: {:#?}", emails);
        // assert_eq!(email_splits, ["Mary", "had", "a", "little", "lamb"]);
        lib::send_email_message(emails, &msg_string.to_string()).await;
    }
}

pub async fn lookup_concur() -> Vec<RespItem> {
    let mut results: Vec<cfg::RespItem> = Vec::new();
    let mut set: JoinSet<RespItem> = JoinSet::new();
    
    for url in cfg::URLS.iter() {
        set.spawn(exec_url_as(url.to_string()));
    }
    
    while let Some(res) = set.join_next().await {
        let out = res;

        let _ = match out {
            Ok(resp) => {
                // println!("success :: {:#?}", resp);
                if resp.code != 200 {
                    results.push(resp);
                }
            },
            Err(err) => {
                println!("error :: {:#?}", err);
            }
        };
    }

    return results;
}

async fn open_data_file() -> File {
    let data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("monitoring.txt")
        .expect("File could not be found/opened");

    data_file
}

async fn append_respitem_to_file(content: &Vec<RespItem>, cur_date: &str) {
    let mut data_file = append_data_to_file("monitoring.txt").await;

    // let content_str: String = content.iter()
    //     .map(|item| item.to_string())
    //     .collect::<Vec<String>>()
    //     .join(",");
    let content_str: String = proc_data_msg(&content);

    // check for empty string
    if content_str.len() > 0 {
        // eprintln!("String length is: {}", content_str.len());
        let data_string: String = format!("{} | {}", cur_date, content_str);

        // newline
        if let Err(e) = writeln!(data_file, "") {
            eprintln!("Couldn't write newline to file: {}", e);
        }
            
        // Write to a file
        data_file
            .write_all(data_string.as_bytes())
            .expect("write failed");
    }
    
}

fn proc_data_msg(content: &Vec<RespItem>) -> String {
    let content_str: String = content.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(",");

    content_str
}

async fn append_data_to_file(path: &str) -> File {
    let data_file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("cannot open file");

    data_file
}


// async fn open_file() {
//     let data_result = File::open("monitoring.json");

//     // Reading a file returns a Result enum
//     // Result can be a file or an error
//     let data_file = match data_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the data file: {:?}", error),
//     };

//     println!("Data file: {:?}", data_file);
// }
