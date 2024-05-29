use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, FromRow};


#[derive(Deserialize, Serialize, Debug, FromRow)]
/// Complete "Edit Suggestion" model as-is in the database
pub struct EditSuggestion {
	pub author_id: String,
	pub post_id: String,
	pub new_content: String,
	pub status: EditStatus
}


#[derive(Deserialize, Serialize, Debug, Type)]
#[sqlx(type_name = "edit_status", rename_all="UPPERCASE")]
/// Complete "Edit Status" enum as-is in the database
pub enum EditStatus {
	PENDING,
	ACCEPTED,
	REJECTED
}

#[derive(Deserialize)]
/// Struct holding fields required from client to create an edit suggestion in the database
pub struct EditSuggestionForCreate {
	pub post_id: String,
	pub new_content: String
}


#[derive(Deserialize)]
/// Struct holding fields required from client to edit an edit_suggestion
pub struct EditSuggestionForEdit {
	pub new_content: Option<String>,
}


#[derive(Serialize, Debug)]
/// Struct holding fields to be sent to the client as a resulting EditSuggestion
pub struct EditSuggestionForResult {
	pub status: EditStatus,
	pub new_content: String
}
