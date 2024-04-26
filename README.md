# axum-cms
A simple production-ready backend server template for Content Management Systems build with Rust and Axum.

## Features
- Authors (Authors of posts)
- Posts (The actual content to be managed)

## Information
- Database used: Postgres
- Cache used: Redis

## Routes
- POST `/signup`: Sign up with name, email, and password
- POST `/login`: Login with email and password
- GET `/author`: get all authors
- GET `/author/:id`: Get specific author.
- PATCH `/author/:id`: Edit specific author.

- GET `/post`: Get all posts
- GET `/post/:id`: Get specific post
- PATCH `/post/:id`: Edit specific post
- DELETE `/post/:id`: Delete specific post

## Instructions
1. Create a .env file in your local copy of the repository, and set the following env variables
	- `PROD_DATABASE_URL`: Your postgres database for production
	- `DEV_DATABASE_URL`: Your postgres database for development
	- `JWT_SECRET`: Your JWT secret
	- `DEV_REDIS_CONN_URL`: Your redis connection url for development, set to 
	- `PROD_REDIS_CONN_URL`: Your redis connection url for production
	- `MODE`: Set as "production"(**CASE SENSITIVE!**) to enable production mode (setting this absent or as any other value will run the program in development mode)
	- `DOCKER_IMAGE_NAME`: The name of your docker image
2. With all your environment variables set, start up your postgres dev server.
```sh
# Default config
# Start postgresql server docker image:
docker run --rm --name pg -p 5433:5433 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

3. Run `cargo run` in your terminal to compile and run your project.
4. To deploy to docker, run the deploy script with the command `./deploy.sh` in the project directory

## Notes
- **IMPORTANT!**: If you decide to change `DEV_DATABASE_URL`, edit the following files accordingly:
	- `sql\dev_initial\00-recreate-db.sql`
	- `src\_dev_utils\dev_db.rs`
- Use the "WithRejection\<`CUSTOM_JSON_BODY`, ApiError>" as Json body type in order to enable JSON extraction errors
- All errors can be found in `src/models/error.rs` in the `Error` enum. You may write custom responses for each error inside the `impl IntoResponse` block for the `Error` enum
