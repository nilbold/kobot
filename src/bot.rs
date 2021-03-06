//! bot definition and event handler

use std::collections::HashSet;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use redis::{self, AsyncCommands};
use serenity::{
   async_trait,
   http::Http,
   model::{
      channel::Message,
      gateway::Ready,
      id::{ChannelId, UserId},
   },
   prelude::*,
};

use crate::command;
use crate::error::{KobotError, Result};

/// bot information meant to be shared inside the message handler
pub struct BotInfo {
   pub id: UserId,
   pub owner: UserId,
   pub token: String,
}

impl TypeMapKey for BotInfo {
   type Value = Arc<BotInfo>;
}

pub struct RedisClient(redis::Client);

impl Deref for RedisClient {
   type Target = redis::Client;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl TypeMapKey for RedisClient {
   type Value = Arc<RedisClient>;
}

pub struct ChannelListen;

impl TypeMapKey for ChannelListen {
   type Value = Arc<RwLock<HashSet<u64>>>;
}

/// `Bot` information and Redis connection
pub struct Bot {
   pub info: Arc<BotInfo>,
   pub redis: Arc<RedisClient>,
   pub listen: Arc<RwLock<HashSet<u64>>>,
}

impl Bot {
   /// create a bot instance
   /// 
   /// sets up the redis client and queries discord for bot info
   pub async fn new<T, U>(token: T, redis_url: U) -> Result<Bot>
   where
      T: Into<String>,
      U: AsRef<str>,
   {
      let token = token.into();
      let redis = redis::Client::open(redis_url.as_ref()).expect("redis client open");

      let listen: HashSet<u64> = {
         let mut con = redis.get_async_connection().await?;

         match con.smembers("listen").await {
            Ok(res) => res,
            Err(why) => {
               eprintln!(
                  "error with redis, could not retrieve listen list: {:?}",
                  why
               );
               return Err(why.into());
            }
         }
      };

      let http = Http::new_with_token(&token);
      let info = http.get_current_application_info().await?;

      Ok(Bot {
         info: Arc::new(BotInfo {
            id: info.id,
            owner: info.owner.id,
            token: token,
         }),
         redis: Arc::new(RedisClient(redis)),
         listen: Arc::new(RwLock::new(listen)),
      })
   }

   /// connect the bot
   /// 
   /// builds and starts the discord client connection
   pub async fn connect(&self) -> Result<()> {
      let mut client = {
         let c = Client::builder(&self.info.token)
            .event_handler(Handler)
            .await;
         c.map_err(KobotError::ClientCreate)?
      };

      {
         let mut data = client.data.write().await;

         data.insert::<BotInfo>(self.info.clone());
         data.insert::<RedisClient>(self.redis.clone());
         data.insert::<ChannelListen>(self.listen.clone());
      }

      client.start().await?;

      Ok(())
   }
}

struct Handler;

impl Handler {
   /// check if kobot should be paying attenion to this channel
   async fn listens_to(&self, context: &Context, channel_id: ChannelId) -> bool {
      let listen = {
         let data_read = context.data.read().await;
         data_read
            .get::<ChannelListen>()
            .expect("ChannelListen in TypeMap")
            .clone()
      };

      let listen = listen.read().unwrap();
      listen.contains(&u64::from(channel_id))
   }
}

#[async_trait]
impl EventHandler for Handler {
   async fn message(&self, context: Context, message: Message) {
      if message.content == "kobot lives here" {
         command::channel_register(&context, &message).await;
         return;
      }

      if !self.listens_to(&context, message.channel_id).await {
         return;
      }

      // making the assumption that all listen channels are guild channels
      let channel_name = message.channel_id.name(&context.cache).await.unwrap();
      println!("#{} > {}", channel_name, message.content);
   }

   async fn ready(&self, _: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);
   }
}

// ex:expandtab sw=3 ts=3
