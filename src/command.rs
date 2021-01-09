use redis::AsyncCommands;
use serenity::{model::channel::Message, prelude::*};

use crate::bot::{BotInfo, ChannelListen, RedisClient};

// kobot lives here
pub async fn channel_register(context: &Context, message: Message) {
   let channel = {
      let channel = match message.channel_id.to_channel(&context).await {
         Ok(channel) => channel,
         Err(why) => {
            eprintln!("error getting channel: {:?}", why);
            return;
         }
      };

      // using .guild() instead of something named like .guild_channel() is a tad
      // confusing, no?
      match channel.guild() {
         Some(guild_channel) => guild_channel,
         None => {
            println!("listen request for non-guild channel, ignoring");
            return;
         }
      }
   };

   // case in point, this gets the actual guild, not just the guild channel,
   // despite the same function name
   let server = match channel.guild(&context.cache).await {
      Some(guild) => guild,
      None => {
         println!("could not find the guild matching this channel, ignoring");
         return;
      }
   };

   let (owner, redis, listen) = {
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
         data_read
            .get::<ChannelListen>()
            .expect("ChannelListen in TypeMap")
            .clone(),
      )
   };

   if message.author.id != owner {
      let dm = message
         .author
         .direct_message(&context, |m| {
            m.content(format!(
               "you're not authorized to control kobot! (from: {})",
               channel.name
            ))
         })
         .await;
      if let Err(why) = dm {
         eprintln!("error sending dm: {:?}", why);
      }

      return;
   }

   {
      let mut con = redis.get_async_connection().await.unwrap();

      let res: i64 = match con.sadd("listen", u64::from(channel.id)).await {
         Ok(res) => res,
         Err(why) => {
            eprintln!("error with redis sadd: {:?}", why);
            return;
         }
      };

      // nothing added to the set? channel is already registered
      if res == 0 {
         let text = "kobot is already listening here!";
         if let Err(why) = channel.say(&context.http, text).await {
            eprintln!("error sending reply: {:?}", why);
         }
         return;
      }
   }

   listen.write().unwrap().insert(u64::from(channel.id));

   {
      let text = format!(
         "yip! kobot now lives in {} {} (listen mode enabled)",
         server.name, channel
      );
      if let Err(why) = channel.say(&context.http, text).await {
         eprintln!("error sending reply: {:?}", why);
         return;
      }
   }

   println!(
      "now listening to {} #{} (enabled by {})",
      server.name, channel.name, message.author.name
   )
}
