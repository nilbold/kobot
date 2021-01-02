use std::sync::Arc;

use serenity::{
   async_trait,
   http::Http,
   model::{channel::Message, gateway::Ready, id::UserId},
   prelude::*,
};

use crate::command;

pub struct BotInfo {
   pub id: UserId,
   pub owner: UserId,
   pub token: String,
}

impl TypeMapKey for BotInfo {
   type Value = Arc<BotInfo>;
}

pub struct Bot {
   pub info: Arc<BotInfo>,
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
         command::channel_register(context, message).await;
      }
   }

   async fn ready(&self, _: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);
   }
}
