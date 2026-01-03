use std::collections::HashMap;

use axum::{Json, Router, extract::{Path, Query, Request, path}, routing::{Route, delete, get, patch, post, put}};
use serde::Deserialize;
use serde_json::{Value,json};


#[tokio::main]

async fn main (){
    let app = app();

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is listning on {} ", listner.local_addr().unwrap());

    axum::serve(listner, app).await.unwrap()

}

//here changed the /hello method to get to get json details

fn app () ->Router {
    Router::new().route("/", get(hello))
    .route("/hello", get(hello))
    .route("/world/{:id}", get(world))
    
}

// use local host and by using 'http://localhost:3000/hello' this
// we can get the below json response there

async fn hello () -> Json<Value> {
    Json(
        json!({
            "name": "Muthu",
            "age": 35
        })
        
    )

   
}

#[derive(Debug,Deserialize)]
struct Personrequest {
    name: String,
    age: i32,
}



async fn world(Path(id): Path<i32>) -> String {
    id.to_string()
}