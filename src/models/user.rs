use crate::tools::UserError;
use actix_identity::Identity;
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, web::Data, Error, FromRequest, HttpRequest,
};
use futures::Future;
use mongodb::bson::oid::ObjectId;
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};
use std::{num::NonZeroU32, pin::Pin, sync::RwLock, u8};

use super::Sessions;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
static SALT_COMPONENT: [u8; 16] = [
    0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a,
];
const PBKDF2_ITER: u32 = 100_000;

#[derive(Deserialize)]
pub struct UserReq {
    username: String,
    password: String,
}

impl UserReq {
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub username: String,
    #[serde(with = "serde_bytes")]
    pub credential: Vec<u8>,
    liked_musics: Vec<i32>,
    current_playlist: Vec<i32>,
    current_playing: i32,
}

impl User {
    pub fn login(&self, user: &UserReq) -> Result<(), UserError> {
        let salt = Self::salt(&user.username);
        let iter = NonZeroU32::new(PBKDF2_ITER).unwrap();
        pbkdf2::verify(
            PBKDF2_ALG,
            iter,
            &salt,
            user.password.as_bytes(),
            &self.credential,
        )
        .map_err(|_| UserError::MismatchingCredential)?;

        Ok(())
    }

    fn salt(username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(SALT_COMPONENT.len() + username.as_bytes().len());
        salt.extend(SALT_COMPONENT.as_ref());
        salt.extend(username.as_bytes());
        salt
    }

    pub fn new(req: &UserReq) -> Self {
        let salt = Self::salt(&req.username);
        let iter = NonZeroU32::new(PBKDF2_ITER).unwrap();
        let mut cred = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(PBKDF2_ALG, iter, &salt, req.password.as_bytes(), &mut cred);
        Self {
            id: None,
            username: req.username.clone(),
            credential: cred.to_vec(),
            liked_musics: Vec::new(),
            current_playing: 0,
            current_playlist: Vec::new(),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_id(&self) -> Option<ObjectId> {
        self.id.clone()
    }

    /// Get a reference to the user's id.
    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }

    /// Get a reference to the user's liked musics.
    pub fn liked_musics(&self) -> &[i32] {
        self.liked_musics.as_ref()
    }

    /// Get a reference to the user's current playlist.
    pub fn current_playlist(&self) -> &[i32] {
        self.current_playlist.as_ref()
    }

    /// Set the user's current playlist.
    pub fn set_current_playlist(&mut self, current_playlist: Vec<i32>) {
        self.current_playlist = current_playlist;
    }

    /// Get a reference to the user's current playing.
    pub fn current_playing(&self) -> i32 {
        self.current_playing
    }

    /// Set the user's current playing.
    pub fn set_current_playing(&mut self, current_playing: i32) {
        self.current_playing = current_playing;
    }
}

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);
        let sessions: Option<&Data<RwLock<Sessions>>> = req.app_data();
        if sessions.is_none() {
            return Box::pin(async { Err(ErrorUnauthorized("unauthorized")) });
        }
        let sessions = sessions.unwrap().clone();
        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                if let Some(user) = sessions
                    .read()
                    .unwrap()
                    .map
                    .get(&identity)
                    .map(|x| x.clone())
                {
                    return Ok(user);
                }
            };

            Err(ErrorUnauthorized("unauthorized"))
        })
    }
}
