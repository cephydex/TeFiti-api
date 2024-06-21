use std::{env, time::Duration};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
// use crate::api;

#[derive(Deserialize, Serialize, Clone)]
struct WigalDest {
    destination: String,
    msgid: i32
}

#[derive(Deserialize, Serialize, Clone)]
struct WigalMsg {
    username: String,
    password: String,
    senderid: String,
    destinations: Vec<WigalDest>,
    // "message": message.replace('[','(').replace(']',')'),
    message: String,
    service: String, //"SMS",
    smstype: String, //"text"
}

impl Default for WigalMsg {    
    fn default() -> WigalMsg {
        let username = env::var("WIGAL_USERNAME").expect("Wigal Username could not be retrieved.");
        // let password = env::var("WIGAL_PASSWORD").expect("Wigal Password could not be retrieved.");
        let sender_id = env::var("SMS_SENDER_ID").expect("SMS Sender ID could not be retrieved.");

        WigalMsg {
            username: username,
            // password: password,
            senderid: sender_id,
            password: "-megF4tun$".to_string(),
            destinations: Vec::new(),
            service: "SMS".to_string(),
            smstype: "text".to_string(),
            message: String::from(""),
        }
    }
}

// pub fn get_list_env2(env_key: &str) -> Vec<String> {
//     let env_emails = env::var(env_key).expect("Admin email list could not be retrieved.");
//     let temp = env_emails.split(";");

//     let mut e_list: Vec<String> = Vec::new();
//     for part in temp {
//         e_list.push(part.to_string());
//     }
//     e_list
// } 

pub fn get_list_env(env_key: &str) -> Vec<String> {
    let env_emails = env::var(env_key).expect("Admin email list could not be retrieved.");
    let emails: Vec<&str> = env_emails.split(";").collect();

    let emails_str = emails.iter().map(|str| str.to_string()).collect();
    emails_str
} 

pub async fn send_message(message: &str) {
    let url = env::var("WIGAL_SMS_URL").expect("Wigal SMS URL could not be retrieved.");
    
    // let client = api::cutil::create_client().await;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build().unwrap();

    let mut dests: Vec<WigalDest> = Vec::new();
    let temp_list = get_list_env("ADMIN_PHONES");
    let phones: Vec<&str> = temp_list.iter().map(|n| n.as_str()).collect();
    println!("Admin phones :: {:#?}", phones);

    // for (i, phone) in ["233249737475", "233202412317"].iter().enumerate() {
    for (i, phone) in phones.iter().enumerate() {
        // let id: i32 = 10 + (i.to_string().parse().unwrap());
        let mut id = i as i32; 
        id += 10;
        dests.push(WigalDest{destination: phone.to_string(), msgid: id});
        println!("{}) PhoneNo :: {}", i, phone);
    }

    let data = WigalMsg {
        destinations: dests,
        message: message.to_string(),
        ..Default::default()
    };

    // SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
    let resp = client
        .post(format!("{}", url))
        // .header("Content-Type", "application/json")
        // .body(json_data)
        .json(&data)
        .send()
        .await;

    if resp.is_ok() {
        let r = resp.as_ref();
        if r.unwrap().status() != StatusCode::OK {
            println!("RES !SMS :: {:#?}", r.unwrap());
        } else {
            println!("RES SMS :: {:#?}", r.unwrap().status());
            // println!("RES !OK :: {}", r.unwrap());
        }
        
    } else if resp.is_err() {
        let rr = resp.as_ref();
        println!("ERR RESP :: {:#?}", rr.err().unwrap());
    }
    
    // return item;
}

#[derive(Deserialize, Serialize)]
struct EmailTo {
    email: String,
    name: String
}

#[derive(Deserialize, Serialize)]
struct ContentItem {  
    r#type: String,
    value: String,  
}

impl Default for ContentItem {
    fn default() -> ContentItem {
        ContentItem {
            r#type: "text/html".to_string(),
            value: String::from("")
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Recipient {
    to: Vec<EmailTo>,
    subject: String,
}

#[derive(Deserialize, Serialize)]
struct FromItem {
    email: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct ReplyToItem {
    email: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct EmailRequest {
	personalizations: Vec<Recipient>,
	content: Vec<ContentItem>,
	from: FromItem,
	reply_to: ReplyToItem
}

pub async fn send_email_message(recipient_emails: Vec<&str>, message: &str) {
    // let url: &str = "https://api.sendgrid.com/v3/mail/send";
    let url = env::var("SENDGRID_URL").expect("Sendgrid URL could not be retrieved.");
    let sendgrid_api_key = env::var("SENDGRID_API_KEY").expect("Sendgrid API key could not be retrieved.");
    let t_sender_email = env::var("FROM_EMAIL").expect("Sendgrid API key could not be retrieved.");
    let t_sender_name = env::var("FROM_NAME").expect("Sendgrid API key could not be retrieved.");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build().unwrap();

    let mut recipient_dets: Vec<EmailTo> = Vec::new();
    for email in recipient_emails {
        recipient_dets.push(EmailTo{email: email.to_owned(), name: email.to_owned()})
    }

    let personalizations: Vec<Recipient> = vec![Recipient{
        subject: "Omni Monitoring".to_string(),
        to: recipient_dets
    }];

    let sender_email: &str = t_sender_email.as_ref();
    let sender_name: &str = t_sender_name.as_ref();

    let email_request = EmailRequest {
        personalizations: personalizations,
        // content: vec![ContentItem{value: message.to_string(), r#type: "text/plain".to_string()}],
        content: vec![ContentItem{value: message.to_string(), ..Default::default()}],
        from: FromItem{email: sender_email.to_string(),  name: sender_name.to_string()},
        reply_to: ReplyToItem{email: sender_email.to_string(),  name: sender_name.to_string()}
    };

    let resp = client
        .post(format!("{}", url))
        .header("Authorization", format!("Bearer {}", sendgrid_api_key))
        .json(&email_request)
        .send()
        .await;

    if resp.is_ok() {
        let r = resp.as_ref();
        if r.unwrap().status() != StatusCode::OK {
            // println!("REQ :: {:#?}", r);
            println!("RES EML :: {:#?}", r.unwrap().status());
        } else {
            // println!("REQ :: {}", url);
            println!("REQ EML :: {}", r.unwrap().status());
        }
        
    } else if resp.is_err() {
        let rr = resp.as_ref();
        println!("ERR RESP :: {:#?}", rr.err().unwrap());
    }
    
    // return item;
}

// use serde_json::{json, Value};
// let payload = json!({
//     "personalizations": [
//         {
//             "to": [
//                 {
//                     "email": recipient_email,
//                     "name": recipient_name
//                 }
//             ],
//             "subject": subject
//         }
//     ],
//     "content": [
//         {
//             "type": "text/plain",
//             "value": html_content
//         }
//     ],
//     "from": {
//         "email": from_email,
//         "name": from_name
//     },
//     "reply_to": {
//         "email": from_email,
//         "name": from_name
//     }
// });


