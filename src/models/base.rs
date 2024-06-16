use std::borrow::Borrow;

use crate::ctx::Ctx;
use crate::models::AppState;
use crate::models::{ModelError, ModelResult};
use sqlb::HasFields;
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::postgres::PgRow;
use sqlx::{Database, Encode, FromRow, Postgres, Type};

/// Trait for Backend Model Controllers that are DB-related
pub trait DbBmc {
	const TABLE: &'static str;
}


pub async fn create<MC, E>(_ctx: &Ctx, app_state: &AppState, data: E) -> ModelResult<i64> 
where
	MC: DbBmc,
	E: Unpin + Send + SqlFields,
{
	
	let db = app_state.db();

	// Get information
	let fields = data.to_field_value_pairs();
	let field_names = format!("({})", get_sql_field_names(&fields).join(", "));
	let field_bind_ids = format!("({})", get_field_bind_ids(&fields).join(", "));
	let field_values = get_sql_field_values(fields);


	let sql = format!("INSERT INTO {} {} VALUES {} returning id", MC::TABLE, field_names, field_bind_ids);

	let mut q = sqlx::query_as::<_, (i64,)>(&sql);

	
	for v in field_values.into_iter() {
		q = v.bind_value_to_query(q)
	}

	let (id, ) = q
		.fetch_one(db)
		.await?;


	Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, app_state: &AppState, id: i64) -> ModelResult<E> 
where
	MC: DbBmc, // ModelController implements DbBmc
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send, // Entity implements FromRow
{

	let db = app_state.db();
	let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);

	let entity: E = sqlx::query_as(&sql)
	.bind(id)
	.fetch_optional(db)
	.await?
	.ok_or(ModelError::EntityNotFound { entity: MC::TABLE, id })?;

	Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, app_state: &AppState) -> ModelResult<Vec<E>> 
where
	MC: DbBmc, // ModelController implements DbBmc
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send, // Entity implements FromRow
{

	let db = app_state.db();
	let sql = format!("SELECT * FROM {} ORDER BY id", MC::TABLE);

	let entity: Vec<E> = sqlx::query_as(&sql)
	.fetch_all(db)
	.await?;

	Ok(entity)
}

#[derive(Debug, Clone)]
pub enum SqlFieldValue {
	I64(i64),
	String(String),
	Bool(bool),
}
pub struct SqlField<'f> {
	pub name: &'f str,
	pub value: SqlFieldValue,
}

impl SqlFieldValue {
	pub fn bind_value_to_query<'a, T>(self, q: sqlx::query::QueryAs<'a, Postgres, T, sqlx::postgres::PgArguments>) -> sqlx::query::QueryAs<'a, Postgres, T, sqlx::postgres::PgArguments> {
		match self {
			Self::Bool(x) => {let q = q.bind(x); q},
			Self::String(x) => {let q = q.bind(x); q},
			Self::I64(x) => {let q = q.bind(x); q},
		}
	}
}

impl<'f> Encode<'f, Postgres> for SqlFieldValue {
	fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'f>>::ArgumentBuffer) -> IsNull {
			use SqlFieldValue as F;
			match self {
					F::I64(v) => Encode::<Postgres>::encode_by_ref(v, buf),
					F::String(v) => Encode::<Postgres>::encode_by_ref(v, buf),
					F::Bool(v) => Encode::<Postgres>::encode_by_ref(v, buf),
			}
	}

	fn encode(self, buf: &mut <Postgres as HasArguments<'f>>::ArgumentBuffer) -> IsNull {
			use SqlFieldValue as F;
			match self {
					F::I64(v) => Encode::<Postgres>::encode(v, buf),
					F::String(v) => Encode::<Postgres>::encode(v, buf),
					F::Bool(v) => Encode::<Postgres>::encode(v, buf),
			}
	}
}

pub trait SqlFields {
	fn to_field_value_pairs(&self) -> Vec<SqlField>;
}

fn get_sql_field_names<'a>(f: &Vec<SqlField<'a>>) -> Vec<&'a str>{
	f.iter()
		.map(|x| x.name)
		.collect::<Vec<&'a str>>()
}

fn get_sql_field_values(f: Vec<SqlField>) -> Vec<SqlFieldValue>{
	f.into_iter()
		.map(|x| x.value.clone())
		.collect::<Vec<_>>()
}

fn get_field_bind_ids(f: &Vec<SqlField>) -> Vec<String> {
	let len = f.len();

	(1..=len)
		.into_iter()
		.map(|i| format!("${}", i))
		.collect::<Vec<_>>()
}