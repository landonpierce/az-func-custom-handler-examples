use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TimerResponse {
   ReturnValue: String,
   Logs: Vec<String>
}


#[tokio::main]
async fn main() {
    let httpExample= warp::get()
        .and(warp::path("api"))
        .and(warp::path("HttpExample"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });


    let timerTriggerExample = warp::post()
    .and(warp::path("TimerTriggerExample"))
    .map(|| {
        let mut res: TimerResponse = TimerResponse { 
            ReturnValue: "Timer Succeeded".to_string(),
            Logs: vec!["Log1".to_string(), "Log2".to_string(), "Log3".to_string()]
        };
        // The functions host expects certain return values and a successful (2xx) HTTP code. 
        // You must also return any log data that you'd like funnelled up to the functions host (especially if you eventually would like it in Azure Monitor)
        // Struct for the response is defined above
        warp::reply::json(&res)
    });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(timerTriggerExample).run((Ipv4Addr::LOCALHOST, port)).await
}