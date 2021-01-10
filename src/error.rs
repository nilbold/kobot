//! error enum definition, wrapping other possible errors

use std::{io, result};

use redis;
use serenity;

use thiserror::Error;

pub type Result<T> = result::Result<T, KobotError>;

/// error returned during bot creation / run
/// 
/// this wraps both Redis and Serenity errors
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

// ex:expandtab sw=3 ts=3
