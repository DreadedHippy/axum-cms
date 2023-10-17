mod dev_db;
use tokio::sync::OnceCell;
use tracing::info;


pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.expect("Could not initialize dev database");
	})
	.await;
}