use thiserror::Error;

#[derive(Error, Debug)]
pub enum KobotError {
   #[error("initialization error")]
   Init,
}
