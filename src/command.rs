use serenity::{model::channel::Message, prelude::*};

use crate::bot::{BotInfo, RedisClient};

// kobot lives here
pub async fn channel_register(context: Context, message: Message) {
   let channel = match message.channel_id.to_channel(&context).await {
      Ok(channel) => channel,
      Err(why) => {
         eprintln!("error getting channel: {:?}", why);
         return;
      }
   };

   let (owner, redis) = {
      let data_read = context.data.read().await;
      (
         data_read
            .get::<BotInfo>()
            .expect("BotInfo in TypeMap")
            .clone()
            .owner,
         data_read
            .get::<RedisClient>()
            .expect("RedisClient in TypeMap")
            .clone(),
      )
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

   {
      let mut _con = redis.get_connection().unwrap();
      // TODO: register channel (redis?)
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

   println!(
      "now listening to {} (enabled by {})",
      channel, message.author.name
   )
}
