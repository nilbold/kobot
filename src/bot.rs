use std::sync::Arc;

use serenity::{
   async_trait,
   http::Http,
   model::{channel::Message, gateway::Ready, id::UserId},
   prelude::*,
};

pub struct BotInfo {
   id: UserId,
   owner: UserId,
   token: String,
}

impl TypeMapKey for BotInfo {
   type Value = Arc<BotInfo>;
}

pub struct Bot {
   info: Arc<BotInfo>,
}

impl Bot {
   pub async fn new(token: String) -> Result<Bot, Box<dyn std::error::Error>> {
      let http = Http::new_with_token(&token);
      let info = http.get_current_application_info().await?;

      Ok(Bot {
         info: Arc::new(BotInfo {
            id: info.id,
            owner: info.owner.id,
            token: token,
         }),
      })
   }

   pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
      let mut client = Client::builder(&self.info.token)
         .event_handler(Handler)
         .await?;

      {
         let mut data = client.data.write().await;
         data.insert::<BotInfo>(self.info.clone());
      }

      client.start().await?;

      Ok(())
   }
}

impl TypeMapKey for Bot {
   type Value = Arc<Bot>;
}

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

         let owner = {
            let data_read = context.data.read().await;
            data_read
               .get::<BotInfo>()
               .expect("BotInfo in TypeMap")
               .clone()
               .owner
         };

         if message.author.id != owner {
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

         println!(
            "now listening to {} (enabled by {})",
            channel, message.author.name
         )
      }
   }

   async fn ready(&self, _: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);
   }
}
