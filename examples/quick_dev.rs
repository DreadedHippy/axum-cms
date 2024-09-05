#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::{json, Value};
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:3000")?;
	let hc_no_auth = httpc_test::new_client("http://localhost:3000")?;
	
	hc.do_get("/hello").await?.print().await?;
	
	// Login hc
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"email": "e@mail",
			"password": "password"
		}),
	);
	
	req_login.await?.print().await?;
	
	// Initialize testers
	let hc_auth_tester = httpc_test::new_client("http://localhost:3000")?;
	let hc_auth_tester2 = httpc_test::new_client("http://localhost:3000")?;

	let req_signup = hc_auth_tester.do_post(
		"/api/signup", 
		json!({
			"email": "e@mail2",
			"password": "password2",
			"name": "Genesis2"
		})
	);
	

	req_signup.await?.print().await?;


	let req_signup = hc_auth_tester2.do_post(
		"/api/signup", 
		json!({
			"email": "e@mail3",
			"password": "password3",
			"name": "Genesis3"
		})
	);
	

	req_signup.await?.print().await?;

	// Check that double sign-up doesn't work
	let req_signup = hc_auth_tester.do_post(
		"/api/signup", 
		json!({
			"email": "e@mail2",
			"password": "password2",
			"name": "Genesis2"
		})
	);

	req_signup.await?.print().await?;
	

	// Login testers
	let req_login_t = hc_auth_tester.do_post(
		"/api/login",
		json!({
			"email": "e@mail2",
			"password": "password2",
		}),
	);

	req_login_t.await?.print().await?;

	let req_login_t2 = hc_auth_tester2.do_post(
		"/api/login",
		json!({
			"email": "e@mail3",
			"password": "password3",
		}),
	);

	req_login_t2.await?.print().await?;

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
	
	let json_value = req_create_post.json_body()?;
	let id = json_value.get("data").and_then(|value| value.get("id")).unwrap();

	// -- Create edit
	let req_create_edit = hc_auth_tester.do_post(
		"/api/edit",
		json!({
			"post_id": id,
			"new_content": "This is just a suggestion"
		})
	);

	let req_create_edit = req_create_edit.await?;
	req_create_edit.print().await?;
	let json_body = req_create_edit.json_body()?;
	let edit_id = json_body.get("data").and_then(|value| value.get("id")).unwrap();
	
	// -- Get created edit
	let edit_id_route = format!("/api/edit/{}", edit_id);
	// check that editor can retrieve edit
	let req_get_edit = hc_auth_tester.do_get(
		&edit_id_route
	);

	req_get_edit.await?.print().await?;

	// check that author can retrieve edit
	let req_get_edit = hc.do_get(
		&edit_id_route
	);
	
	req_get_edit.await?.print().await?;

	// check that third party cannot retrieve edit
	let req_get_edit = hc_auth_tester2.do_get(
		&edit_id_route
	);
	
	req_get_edit.await?.print().await?;

	// -- Update created edit
	// check that editor can update edit
	let req_update_edit = hc_auth_tester.do_patch(
		&edit_id_route,
		json!({
			"new_content": "This is just an updated suggestion"
		})
	);
	
	req_update_edit.await?.print().await?;

	// check that post author cannot update edit
	let req_update_edit = hc.do_patch(
		&edit_id_route,
		json!({
			"new_content": "This is just an updated suggestion"
		})
	);
	
	req_update_edit.await?.print().await?;

	// check that third party cannot update edit
	let req_update_edit = hc_auth_tester2.do_patch(
		&edit_id_route,
		json!({
			"new_content": "This is just an updated suggestion"
		})
	);
	
	req_update_edit.await?.print().await?;

	// -- Delete created edit
	// check that post author cannot delete edit
	let req_delete_edit = hc.do_delete(
		&edit_id_route
	);
	
	req_delete_edit.await?.print().await?;

	// check that third party cannot delete edit
	let req_delete_edit = hc_auth_tester2.do_delete(
		&edit_id_route
	);

	req_delete_edit.await?.print().await?;
	
	// check that editor can delete edit
	let req_delete_edit = hc_auth_tester.do_delete(
		&edit_id_route
	);
	
	req_delete_edit.await?.print().await?;
	
	// -- Re-create edit
	let req_create_edit = hc_auth_tester.do_post(
		"/api/edit",
		json!({
			"post_id": id,
			"new_content": "This is just a suggestion"
		})
	);

	let req_create_edit = req_create_edit.await?;
	req_create_edit.print().await?;

	// -- List outgoing edits by `hc_auth_tester`
	let req_list_outgoing_edits = hc_auth_tester.do_get(
		"/api/edit/outgoing"
	);

	req_list_outgoing_edits.await?.print().await?;

	// -- List incoming edits for `hc`
	let req_list_incoming_edits = hc.do_get(
		"/api/edit/incoming"
	);

	req_list_incoming_edits.await?.print().await?;

	// -- Accept edit
	let json_body = req_create_edit.json_body()?;
	let edit_id = json_body.get("data").and_then(|value| value.get("id")).unwrap();
	let accept_edit_route = format!("/api/edit/accept/{}", edit_id);
	let req_accept_edit = hc.do_post(
		&accept_edit_route,
		json!({
			"accept": true
		})
	);
	req_accept_edit.await?.print().await?;

	let edit_id_route = format!("/api/edit/{}", edit_id);
	// -- Check that editor cannot update a non-pending edit
	let req_update_edit = hc_auth_tester.do_patch(
		&edit_id_route,
		json!({
			"new_content": "This is an update after author's verdict"
		})
	);

	req_update_edit.await?.print().await?;

	// -- Check that editor cannot delete an accepted edit
	let req_delete_edit = hc_auth_tester.do_delete(
		&edit_id_route
	);

	req_delete_edit.await?.print().await?;

	// -- Double Accept should fail
	let req_accept_edit = hc.do_post(
		&accept_edit_route,
		json!({
			"accept": true
		})
	);

	req_accept_edit.await?.print().await?;
	
	// -- Update post
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

	
	// -- Authors

	// -- List authors
	let req_list_authors = hc_no_auth.do_get("/api/author");

	let req_list_authors = req_list_authors.await?;

	req_list_authors.print().await?;

	// -- Get specific author
	let json_body = req_list_authors.json_body()?;
	let id = json_body.get("data").and_then(|v| v.get(0)).and_then(|v| v.get("id")).unwrap();
	let get_author_route = format!("/api/author/{}", id);
	let req_get_author = hc_no_auth.do_get(&get_author_route);

	req_get_author.await?.print().await?;


	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);


	req_logoff.await?.print().await?;


	Ok(())
}