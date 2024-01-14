use std::{collections::HashMap, fs};

use axum::{
    routing::get,
    response::Json,
    Router,
    extract::Query,
};
use serde_json::{Value,json};
#[tokio::main]
async fn main(){
    let app = Router::new()
                    .route("/",get(|| async{"Hello, this is an api to get daily noble thoughts\n"}))
                    .route("/api",get(arr_of_msg_gen));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1819").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
async fn arr_of_msg_gen(Query(params):Query<HashMap<String,String>>)-> Json<Value>{
    let date = format!("days/{}.txt",params["date"]);
    println!("{}",date);
    let content = fs::read_to_string(date).unwrap();
    let mut out_json = json!(
        {"date":params["date"]}
    );
    let mut cur = 0;
    let mut msg = "".to_string();
    for line in content.lines(){
        let mut char_it = line.chars();
        msg+=line;
        while let Some(c) = char_it.next(){
            if c=='m'{
                let rest_str:String = char_it.clone().collect();
                if rest_str=="sgStart "{
                    println!("{}",rest_str);
                    msg = msg[0..msg.len()-10].to_string();
                    if cur!=0 {out_json[format!("{}",cur)] = Value::String(msg);}
                    msg = "".to_string();
                    cur+=1;
                    break;
                }
            }
        }
    }
    out_json[format!("{}",cur)] = Value::String(msg.to_string());
    Json(out_json)
} 
