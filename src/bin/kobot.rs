//! kobot

use std::env;

fn main() {
   println!("kobot {}", env!("GIT_VERSION"));

   //let _args = &env::args_os().collect::<Vec<_>>();

   let token = env::var("DISCORD_TOKEN").expect("discord token");

   if let Err(why) = kobot::run(&token) {
      eprintln!("error during bot run: {:?}", why);
      std::process::exit(1);
   }
}

// ex:expandtab sw=3 ts=3
