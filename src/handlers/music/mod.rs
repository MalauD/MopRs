mod album;
mod artist;
mod cdn;
mod like;
mod music;
mod playlist;
mod related;
mod search;
mod tools;
mod trending;

pub use self::{
    album::*, artist::*, cdn::*, like::*, music::*, playlist::*, related::*, search::*, tools::*,
    trending::*,
};
