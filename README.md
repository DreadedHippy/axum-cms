# axum-cms
A simple production-ready backend server template for Content Management Systems build with Rust and Axum.

## Features
- Authors (Authors of posts)
- Posts (The actual content to be managed)

## Information
- Database used: Postgres
- Cache used: Redis

## Routes

## Instructions
Create a .env file in your local copy of the repository, and set the following env variables
- `PROD_DATABASE_URL`: Your postgres database for production
- `DEV_DATABASE_URL`: Your postgres database for development
- `JWT_SECRET`: Your JWT secret
- `DEV_REDIS_CONN_URL`: Your redis connection url for development
- `PROD_REDIS_CONN_URL`: Your redis connection url for production
- `MODE`: Set as "production"(**CASE SENSITIVE!**) to enable production mode (setting this absent or as any other value will run the program in development mode)

## Notes
- Use the "WithRejection\<`CUSTOM_JSON_BODY`, ApiError>" in order to enable JSON extraction errors
- All errors can be found in `src/models/error.rs` in the `Error` enum, you may write custom responses for each error inside the `impl IntoResponse` block for the `Error` enum
