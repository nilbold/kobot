use tokio::runtime::Runtime;

use bot::Bot;

mod bot;

pub fn run<T: AsRef<str>>(token: T) -> Result<(), Box<dyn std::error::Error>> {
   let mut rt = Runtime::new()?;

   rt.block_on(async {
      let bot = Bot::new(token.as_ref().into()).await?;

      bot.connect().await
   })
}

// ex:expandtab sw=3 ts=3
