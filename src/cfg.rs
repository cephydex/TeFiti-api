use std::{fmt, time::SystemTime};

use time::{format_description, OffsetDateTime};

pub const DATE_FORMAT_STR: &'static str = "[year]-[month]-[day]-[hour]:[minute]:[second]";

pub const URLS:[&str; 13] = [
    "http://omnistrategies.net",
    "http://akofisgroup.com",
    "http://akofisengineering.com",

    "https://megafortunelottery.com",
    "https://api.megafortunelottery.com",
    "https://public-api.megafortunelottery.com/swagger/index.html",
    "https://worker.megafortunelottery.com/",
    "https://backend.megafortunelottery.com",
    "https://admin.megafortunelottery.com/",

    "https://backend.mypolicy.market/",
    "https://api.mypolicy.market",
    "https://admin.mypolicy.market",
    "https://temp.mypolicy.market",
    // "https://mypolicy.market",
    // "https://datacollection.mypolicy.market/",
];

pub struct RespItem {
    // pub site: &'static str,
    pub site: String,
    pub code: i32
}

impl fmt::Debug for RespItem {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RespItem")
            .field("site", &self.site)
            .field("code", &format_args!("{}", self.code))
            .finish()
    }
}

impl fmt::Display for RespItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b_str: String = "{".to_owned();
        let e_str: String = "}".to_owned();
        let fm1 = format!("Site: {}, Code: {}", self.site, self.code);
        let final_str = format!("{b_str}{fm1}{e_str}");
        write!(f, "{},", final_str)
        // write!(f, "Site: {}, Code: {}", self.site, self.code)
    }
}

pub fn print_date() -> (String, String) {
    let dt1: OffsetDateTime = SystemTime::now().into();
    let dt2 = OffsetDateTime::now_utc();

    let dt_fmt = format_description::parse(DATE_FORMAT_STR).unwrap();
    let dt1_str = dt1.format(&dt_fmt).unwrap();
    // let dt2_str = dt2.format(&dt_fmt).unwrap();

    println!("INIT :: {}", dt1_str);
    (dt1_str, dt2.format(&dt_fmt).unwrap())
}