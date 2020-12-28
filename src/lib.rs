use serenity::{
   async_trait,
   model::{channel::Message, gateway::Ready},
   prelude::*,
};

use tokio::runtime::Runtime;

use bot::Bot;

mod bot;

// nil#1337
const CREATOR_ID: u64 = 124335242176757766;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
   async fn message(&self, context: Context, message: Message) {
      if message.content == "kobot lives here" {
         let channel = match message.channel_id.to_channel(&context).await {
            Ok(channel) => channel,
            Err(why) => {
               eprintln!("error getting channel: {:?}", why);
               return;
            }
         };

         if message.author.id != CREATOR_ID {
            let dm = message
               .author
               .direct_message(&context, |m| {
                  m.content(format!(
                     "you're not authorized to control kobot! (from: {})",
                     channel
                  ))
               })
               .await;
            if let Err(why) = dm {
               eprintln!("error sending dm: {:?}", why);
            }

            return;
         }

         let reply = message
            .channel_id
            .say(
               &context.http,
               format!("yip! kobot now lives in {} (listen mode enabled)", channel),
            )
            .await;
         if let Err(why) = reply {
            eprintln!("error sending reply: {:?}", why);

            return;
         }

         // TODO: register channel (redis?)

         println!("now listening to {} (enabled by {})", channel, message.author.name)
      }
   }

   async fn ready(&self, _: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);
   }
}

pub fn run<T: AsRef<str>>(token: T) -> Result<(), Box<dyn std::error::Error>> {
   let mut rt = Runtime::new()?;

   rt.block_on(async {
      let _bot = Bot::new(token.as_ref().into()).await?;

      let mut client = Client::builder(&token).event_handler(Handler).await?;

      client.start().await?;


      Ok(())
   })
}

// ex:expandtab sw=3 ts=3
