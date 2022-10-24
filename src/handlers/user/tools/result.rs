use actix_web::HttpResponse;

use crate::tools::UserError;

pub type UserResponse = Result<HttpResponse, UserError>;
