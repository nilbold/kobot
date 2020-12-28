use serenity::{
   model::{id::UserId},
   http::Http,
};

pub struct Bot {
   id: UserId,
   owner: UserId,
   token: String,
}

impl Bot {
   pub async fn new(token: String) -> Result<Bot, Box<dyn std::error::Error>> {
      let http = Http::new_with_token(&token);
      let info =  http.get_current_application_info().await?;

      Ok(Bot {
         id: info.id,
         owner: info.owner.id,
         token: token,
      })
   }
}
