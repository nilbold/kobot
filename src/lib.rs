use tokio::runtime::Runtime;

use bot::Bot;
use error::{KobotError, Result};

mod bot;
mod command;
mod error;

/// create and run the bot
/// 
/// iniitlizes the async runtime before connecting to discord and redis
pub fn run<T, U>(token: T, redis_url: U) -> Result<()>
where
   T: AsRef<str>,
   U: AsRef<str>,
{
   let mut rt = Runtime::new().map_err(KobotError::Init)?;

   rt.block_on(async {
      let bot = Bot::new(token.as_ref(), redis_url).await?;

      bot.connect().await
   })
}

// ex:expandtab sw=3 ts=3
