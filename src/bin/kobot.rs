//! kobot

use std::env;

pub fn main() {
   println!("kobot {}", env!("GIT_VERSION"));

   let _args = &env::args_os().collect::<Vec<_>>();

   kobot::run();
}

// ex:expandtab sw=3 ts=3
