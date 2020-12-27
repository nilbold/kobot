
use serenity::{
   async_trait,
   model::{channel::Message, gateway::Ready},
   prelude::*,
};

use tokio::runtime::Runtime;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
   async fn message(&self, ctx: Context, msg: Message) {
      if msg.content == "!ping" {
         if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
            eprintln!("error sending message: {:?}", why);
         }
      }
   }

   async fn ready(&self, _: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);
   }
}

pub fn run<T: AsRef<str>>(token: T) -> Result<(), Box<dyn std::error::Error>> {
   let mut rt = Runtime::new()?;

   rt.block_on(async {
      let mut client = Client::builder(&token)
         .event_handler(Handler)
         .await?;

      client.start().await?;

      Ok(())
   })
}

// ex:expandtab sw=3 ts=3
