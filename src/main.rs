use axum::{
    routing::get,
    routing::post,
    Router,
};

#[tokio::main]
async fn main(){
    let app = Router::new()
                    .route("/",get(|| async{"Hello, this is an api to get daily noble thoughts"}))
                    .route("/api/:date",post(arr_of_msg_gen));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1819").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
async fn arr_of_msg_gen(){
    
} 