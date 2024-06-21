use serde::{Deserialize, Serialize};
// use warp::reject::Rejection;
// use warp::reply::Reply;
// use warp::hyper::StatusCode;

#[derive(Deserialize, Serialize, Clone)]
struct Resp {
    message: String
}

pub async fn index_page() -> Result<impl warp::Reply, warp::Rejection> {
    let r = Resp {message: String::from("Te Fiti Server is live!")};    
    Ok(warp::reply::json(&r))
}

// pub async fn handle_not_found(reject: Rejection) -> Result<impl Reply, Rejection> {
//     if reject.is_not_found() {
//         Ok(StatusCode::NOT_FOUND)
//     } else {
//         Err(reject)
//     }
// }

// pub async fn routes() -> Result<impl warp::Reply, warp::Rejection> {

//     let index2 = warp::get()
//             .and(warp::path("v1"))
//             .and(warp::path::end())
//             .and_then(index_page);
    
//     let hello = warp::path!("hello" / String)
//         .map(|name| format!("hello {}", name));

//     let index1 = warp::get()
//         .and(warp::path::end())
//         .and_then(index_page);

//     let download_readme = warp::get()
//         .and(warp::path("v1"))
//         .and(warp::path("files"))
//         .and(warp::path("readme.md"))
//         .and(warp::path::end())
//         .and(warp::fs::file("./README.md"));

//     let download_monitor = warp::get()
//         .and(warp::path("v1"))
//         .and(warp::path("files"))
//         .and(warp::path("monitor.txt"))
//         .and(warp::path::end())
//         .and(warp::fs::file("./monitoring.txt"));

//     let routes = hello
//         .or(download_readme)
//         .or(download_monitor)
//         .or(index1)
//         .or(index2);

//     // let server = warp::serve(routes)
//     //     // warp::serve(api::routes::routes)
//     //         .run(([127, 0, 0, 1], 3003));
//     // return server
//     routes
// }
