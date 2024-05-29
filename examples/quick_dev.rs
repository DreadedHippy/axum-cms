
#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hmmm").await?.print().await?; // Fallback route works fine
    
    
    hc.do_get("/author").await?.print().await?; // Get all authors route works fine

    hc.do_post("/signup", json!({
      "name": "Author2",
      "email": "email@email.com",
      "password": "password123"
    })).await?.print().await?; // Signup route(for author) works fine

    hc.do_post("/signup", json!({
      "name": "Author3",
      "email": "email@email.com",
      "password": "password123"
    })).await?.print().await?; // Double signup throws an error as expected

    hc.do_post("/login", json!({
      "email": "email@email.com123",
      "password": "password123"
    })).await?.print().await?; // Login route with invalid email throws the expected "NOT_FOUND" response
    

    hc.do_post("/login", json!({
      "email": "email@email.com",
      "password": "password12345"
    })).await?.print().await?; // Login route with bad password credential throws an error as expected


    let user_login = hc.do_post("/login", json!({
      "email": "email@email.com",
      "password": "password123"
    })).await?; 

    user_login.print().await?; // Login route with proper credentials logs user in and sets cookies as expected

    // Parse the ID from json response
    let json_response = user_login.json_body()?;
    let json_user_id = &json_response["data"].as_object().expect("Could not get ID from Login JSON response")["id"];

    let user_id = format!("{}", json_user_id).parse::<i64>()?; // Fetch the user ID as an i64

    hc.do_get(format!("/author/{}", user_id).as_str()).await?.print().await?; // Get specific author route works fine





    Ok(())
}
