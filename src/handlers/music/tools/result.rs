use actix_web::HttpResponse;

use crate::tools::MusicError;

pub type MusicResponse = Result<HttpResponse, MusicError>;
