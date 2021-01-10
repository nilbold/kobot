use std::{io, result};

use redis;
use serenity;

use thiserror::Error;

pub type Result<T> = result::Result<T, KobotError>;

#[derive(Error, Debug)]
pub enum KobotError {
   #[error("initialization error")]
   Init(#[source] io::Error),
   #[error("could not create bot client")]
   ClientCreate(#[source] serenity::Error),
   #[error("serenity error")]
   Serenity(#[from] serenity::Error),
   #[error("redis error")]
   Redis(#[from] redis::RedisError),
}
