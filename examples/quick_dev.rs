#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:3000")?;
	let hc_no_auth = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"email": "e@mail",
			"password": "password"
		}),
	);

	req_login.await?.print().await?;

	// -- Create post
	let req_create_post = hc.do_post(
		"/api/post", 
		json!({
			"title": "First post",
			"content": "First post content",
			"author_id": 1000
		}),
	);

	let req_create_post = req_create_post.await?;
	
	req_create_post.print().await?;
	
	// -- Update post
	let json_value = req_create_post.json_body()?;
	let id = json_value.get("data").and_then(|value| value.get("id")).unwrap();

	let update_route = format!("/api/post/{}", id);
	let req_update_post = hc.do_patch(
		&update_route,
		json!({
			"title": "First post title edited",
			"content": "First post content edited",
		}),
	);

	req_update_post.await?.print().await?;

	// -- List all posts
	let req_list_posts = hc_no_auth.do_get(
		"/api/post"
	);

	req_list_posts.await?.print().await?;
	
	// -- Delete post
	let delete_route = format!("/api/post/{}", id);
	let req_delete_post = hc.do_delete(
		&delete_route
	);
	
	req_delete_post.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	req_logoff.await?.print().await?;


	Ok(())
}