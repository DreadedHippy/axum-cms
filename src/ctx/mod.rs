
type CtxResult<T> = Result<T, CtxError>;
#[derive(Clone, Debug)]
pub enum CtxError {
	CtxCannotNewRootCtx
}

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64
}


impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx {user_id: 0}
	}
	
	pub fn new(user_id: i64) -> CtxResult<Self> {
		if user_id == 0 {
			Err(CtxError::CtxCannotNewRootCtx)
		} else {
			Ok( Self {user_id})
		}
	}
}

impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}