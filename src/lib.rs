use tokio::runtime::Runtime;

use bot::Bot;
use error::KobotError;

mod bot;
mod command;
mod error;

pub fn run<T, U>(token: T, redis_url: U) -> Result<(), Box<dyn std::error::Error>>
where
   T: AsRef<str>,
   U: AsRef<str>,
{
   let mut rt = Runtime::new()?;

   rt.block_on(async {
      let bot = Bot::new(token.as_ref(), redis_url).await?;

      bot.connect().await
   })
}

// ex:expandtab sw=3 ts=3
