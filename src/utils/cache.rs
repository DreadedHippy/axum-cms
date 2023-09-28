use std::env;

// use redis::{Client, RedisError, Connection, Commands};
use redis::{aio::Connection, AsyncCommands, FromRedisValue};
use anyhow::Result;
// use crate::models::error::{Result, Error};

// pub async fn create_redis_client() -> Result<Connection>{
// 	let redis_hostname = env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
// 	let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

// 	let redis_conn_url = format!("rediss://:{}@{}", redis_password, redis_hostname);

// 	println!("Connecting to redis...");

// 	let mut conn = redis::Client::open(redis_conn_url)
// 		.expect("invalid connection URL")
// 		.get_async_connection()
// 		.await
// 		.expect("failed to connect to redis");
// 	// throw away the result, just make sure it does not fail
// 	// let _: () = redis::cmd("SET")
// 	// 	.arg("key_1")
// 	// 	.arg("value_1")
// 	// 	.execute(&mut conn);

// 	let _: () = conn.set("key_1", "value_1").await?;

// 	println!("Connected");

// 	Ok(conn)
// }



pub async fn create_redis_connection() -> Result<Connection>{
	println!("Connecting to redis");
	let redis_host_name = env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
	let redis_password = env::var("REDIS_PASSWORD").expect("missing environment variable REDIS_PASSWORD");
	let redis_conn_url = format!("rediss://:{}@{}", redis_password, redis_host_name);

	let  mut conn = redis::Client::open(redis_conn_url)
			.expect("invalid connection URL")
			.get_tokio_connection()
			.await
			.expect("failed to connect to redis");

	
	// let _: () = redis::cmd("SET")
	// .arg("foo")
	// .arg("bar")
	// .query(&mut conn)
	// .expect("failed to execute SET for 'foo'");

	


	
	let bar: String = conn.get("foo").await?;

	println!("value for 'foo' = {}", bar);



	println!("Connected");

	Ok(conn)

	
}


