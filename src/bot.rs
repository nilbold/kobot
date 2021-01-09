use std::collections::HashSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
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

#[derive(Debug, Clone)]
struct BotInitError;

impl Display for BotInitError {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "could not initialize the bot")
   }
}

impl Error for BotInitError {}

pub struct Bot {
   pub info: Arc<BotInfo>,
   pub redis: Arc<RedisClient>,
   pub listen: Arc<RwLock<HashSet<u64>>>,
}

impl Bot {
   pub async fn new<T, U>(token: T, redis_url: U) -> Result<Bot, Box<dyn Error>>
   where
      T: Into<String>,
      U: AsRef<str>,
   {
      let token = token.into();
      let redis = redis::Client::open(redis_url.as_ref()).expect("redis client open");

      let listen: HashSet<u64> = {
         let mut con = redis.get_async_connection().await.unwrap();

         match con.smembers("listen").await {
            Ok(res) => res,
            Err(why) => {
               eprintln!(
                  "error with redis, could not retrieve listen list: {:?}",
                  why
               );
               return Err(BotInitError.into());
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

   pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
      let mut client = Client::builder(&self.info.token)
         .event_handler(Handler)
         .await?;

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
         command::channel_register(&context, message).await;
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
