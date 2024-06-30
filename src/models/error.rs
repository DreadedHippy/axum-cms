use crate::models::store;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type ModelResult<T> = core::result::Result<T, ModelError>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum ModelError {
	EntityNotFound { entity: &'static str, id: i64 },
	// -- Modules
	Store(store::StoreError),

	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
	SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error)
}

// region:    --- Froms
impl From<sqlx::Error> for ModelError {
	fn from(val: sqlx::Error) -> Self {
		Self::Sqlx(val)
	}
}

impl From<sea_query::error::Error> for ModelError {
	fn from(val: sea_query::error::Error) -> Self {
		Self::SeaQuery(val)
	}
}

impl From<store::StoreError> for ModelError {
	fn from(val: store::StoreError) -> Self {
		Self::Store(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for ModelError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for ModelError {}
// endregion: --- Error Boilerplate