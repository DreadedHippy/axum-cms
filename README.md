# axum-cms
A simple production-ready backend server template for Content Management Systems built with Rust using Axum.
!!! Go through the `Critical Information 🚨🚨🚨` section below before getting started !!!

## Tools
- [Axum](https://docs.rs/axum/latest/axum/)
- [Postgresql](https://www.postgresql.org/)
- [SQLX](https://docs.rs/sqlx/latest/sqlx/)
- [SeaQuery](https://docs.rs/sea-query/latest/sea_query/)
- [ModQL](https://docs.rs/modql/latest/modql/)
- [SerdeJSON](https://docs.rs/serde_json/latest/serde_json/)

## Features
- Authors (Authors of posts)
- Posts (The actual content to be managed)
- EditSuggestion (Edits on posts suggested by self or other authors)
- Redis Caching (Discontinued, may be re-implemented in the future)
- WebSocket Draft Saves (Coming Soon, perhaps)
- Google sign-up/sign-in (Coming soon, perhaps)
- Refactored for even quicker development 🚀🚀🚀. Happy coding 💫

## Critical Information 🚨🚨🚨
- Make sure to set the correct config information in [the config file](.cargo/config.toml).
- For production, Make sure to comment out `_dev_utils::init_dev().await?` in the `main()` function as this is for dev only.
- For development, In all `.sql` files, individual database statements should end with `;--#`, failure to do this may break dev database initialization
- If you have a field with a database enum, I advise avoiding `base`'s generic `update` method. This breaks with postgres at the moment, you would have to manually cast the enum fields to a database enum using sea_query. See the `update` method at [edit.rs](src/models/edit.rs) for a detailed example.

## Routes
See [here](src/web/routes/mod.rs) for the code implementation of all the routes
### Auth
- POST `/signup`: Sign up with name, email, and password
- POST `/login`: Login with email and password

### Author
- GET `/api/author`: get all authors
- GET `/api/author/:id`: Get specific author.
<!-- - PATCH `/author/:id`: Edit specific author. -->

### Post
- GET `/api/post`: Get all posts
- GET `/api/post/:id`: Get specific post
- PATCH `/api/post/:id`: Update specific post
- DELETE `/api/post/:id`: Delete specific post

### Edit
- GET `/api/edit`: List edits by/for an author's posts
- GET `/api/edit/incoming`: List all incoming edits
- GET `/api/edit/outgoing`: List all outgoing edits
- GET `/api/edit/:id`: Get edit
- PATCH `/api/edit/:id`: Update edit
- DELETE `/api/edit/:id`: Delete edit
- POST `/api/edit/accept/:id` Accept edit
- POST `/api/edit/reject/:id` Reject edit

## Testing
## Unit Tests
You must have cargo-watch installed for these to work
```sh
# Run tests
cargo watch -q -c -x "test -- -- nocapture"

# Specific test with filter
cargo watch -q -c -x "test models::author::tests::test_create_ok"

# Run quick_dev exampl while developing
cargo watch -q -c -w examples/ -x "run --example quick_dev"

# Run any example while developing
cargo watch -q -c -w examples/ -x "run --example {FILE_NAME}"` # where `FILE_NAME` is the name of the file containing the test
```


## Notes
- **IMPORTANT!**: If you decide to change `DEV_DATABASE_URL`, edit the following files accordingly:
	- `sql\dev_initial\00-recreate-db.sql`
	- `src\_dev_utils\dev_db.rs`
- Use the "WithRejection<`CUSTOM_JSON_BODY`, ApiError>" as Json body type in order to enable JSON extraction errors
- ~~All errors can be found in `src/models/error.rs` in the `Error` enum. You may write custom responses for each error inside the `impl IntoResponse` block for the `Error` enum~~
- All fixtures are prefixed with 'fx'
- An e2e example is given in the `/examples` folder 
- Run the example with the command: `cargo run --example {FILE_NAME}`, where `FILE_NAME` is the name of the file containing the example, in this case, `quick_dev`
- With `cargo watch` installed, you can automatically re-run the example on each file save with the command: `cargo watch -q -c -w examples/ -x "run --example {FILE_NAME}"` instead.
<!-- ## Instructions
1. Setup your configuration in [the config file](.cargo/config.toml).
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

 -->

<!-- ## Notes
- **IMPORTANT!**: If you decide to change `DEV_DATABASE_URL`, edit the following files accordingly:
	- `sql\dev_initial\00-recreate-db.sql`
	- `src\_dev_utils\dev_db.rs`
- Use the "WithRejection\<`CUSTOM_JSON_BODY`, ApiError>" as Json body type in order to enable JSON extraction errors
- All errors can be found in `src/models/error.rs` in the `Error` enum. You may write custom responses for each error inside the `impl IntoResponse` block for the `Error` enum
- All fixtures are prefixed with 'fx'
- Tests are stored in: `/examples` folder 
- Run the tests with the command: `cargo run --example {FILE_NAME}`, where `FILE_NAME` is the name of the file containing the test
- With `cargo watch` installed, Re-run the test on each file save with the command: `cargo watch -q -c -w examples/ -x "run --example {FILE_NAME}"`, where `FILE_NAME` is the name of the file containing the test -->
