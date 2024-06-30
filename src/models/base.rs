use std::borrow::Borrow;
use modql::field::HasFields;
use modql::SIden; // "SIden" stands for "Static Identifier"
use sea_query::{Expr, Iden, IntoIden, PostgresQueryBuilder, Query, TableRef};
use sea_query_binder::SqlxBinder;
use crate::ctx::Ctx;
use crate::models::AppState;
use crate::models::{ModelError, ModelResult};
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::postgres::PgRow;
use sqlx::{Database, Encode, FromRow, Postgres, Type};

/// Trait for Backend Model Controllers that are DB-related
pub trait DbBmc {
	const TABLE: &'static str;

	fn table_ref() -> TableRef {
		TableRef::Table(SIden(Self::TABLE).into_iden())
	}
}

#[derive(Iden)]
pub enum CommonIden {
	Id
}


pub async fn create<MC, E>(_ctx: &Ctx, app_state: &AppState, data: E) -> ModelResult<i64> 
where
	MC: DbBmc,
	E: Unpin + Send + HasFields,
{
	
	let db = app_state.db();

	// -- Extract fields (name/ sea-query value expression)
	let fields = data.not_none_fields();
	let (columns, sea_values) = fields.for_sea_insert();

	// -- Build query
	let mut query = Query::insert();

	query
		.into_table(MC::table_ref())
		.columns(columns)
		.values(sea_values)?
		.returning(Query::returning().columns([CommonIden::Id]));


	// -- Execute query
	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

	let (id,) = sqlx::query_as_with::<_, (i64,), _>(&sql, values)
		.fetch_one(db)
		.await?;

	Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, app_state: &AppState, id: i64) -> ModelResult<E> 
where
	MC: DbBmc, // ModelController implements DbBmc
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send, // Entity implements FromRow
	E: HasFields
{

	let db = app_state.db();

	// -- Build query
	let mut query = Query::select();

	query
		.from(MC::table_ref())
		.columns(E::field_column_refs())
		.and_where(Expr::col(CommonIden::Id).eq(id));


	// -- Execute query
	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let entity = sqlx::query_as_with::<_, E, _>(&sql, values)
		.fetch_optional(db)
		.await?
		.ok_or(ModelError::EntityNotFound {
			entity: MC::TABLE,
			id
		})?;

	

	Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, app_state: &AppState) -> ModelResult<Vec<E>> 
where
	MC: DbBmc, // ModelController implements DbBmc
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send, // Entity implements FromRow
	E: HasFields
{

	let db = app_state.db();

	// -- Build Query
	let mut query = Query::select();
	query.from(MC::table_ref()).columns(E::field_column_refs());
	
	// -- Execute Query
	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let entities = sqlx::query_as_with::<_, E, _>(&sql, values)
		.fetch_all(db)
		.await?;

	Ok(entities)
}


pub async fn update<MC, E>(
	_ctx: &Ctx, 
	app_state: &AppState, 
	id: i64,
	data: E
) -> ModelResult<()> 
where
	MC: DbBmc,
	E: HasFields,
{
	
	let db = app_state.db();

	// -- Prep data
	let fields = data.not_none_fields();
	let fields = fields.for_sea_update();

	// -- Build query
	let mut query = Query::update();
	query
		.table(MC::table_ref())
		.values(fields)
		.and_where(Expr::col(CommonIden::Id).eq(id));

	// -- Execute query
	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let count = sqlx::query_with(&sql, values)
		.execute(db)
		.await?
		.rows_affected();

	// -- Check result
	if count == 0 {
		Err(ModelError::EntityNotFound { entity: MC::TABLE, id })
	} else {
		Ok(())
	}
}

pub async fn delete<MC>(_ctx: &Ctx, app_state: &AppState, id: i64) -> ModelResult<()> 
where
	MC: DbBmc
{
	let db = app_state.db();

	// Build query
	let mut query = Query::delete();

	query
		.from_table(MC::table_ref())
		.and_where(Expr::col(CommonIden::Id).eq(id));

	// Execute query
	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let count = sqlx::query_with(&sql, values)
		.execute(db)
		.await?
		.rows_affected();

	// Check result
	if count == 0 {
		Err(ModelError::EntityNotFound { entity: MC::TABLE, id })
	} else {
		Ok(())
	}

}
