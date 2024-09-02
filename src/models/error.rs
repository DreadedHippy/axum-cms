use crate::{crypt, models::store};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use derive_more::{From, Display};

pub type ModelResult<T> = core::result::Result<T, ModelError>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum ModelError {
	EntityNotFound { entity: &'static str, id: i64 },

	EntityAccessRequiresAuth,

	// -- Modules
	#[from]
	Crypt(crypt::CryptError),
	#[from]
	Store(store::StoreError),

	// -- Externals
	#[from]
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
	#[from]
	SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
	#[from]
	ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError)
}

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