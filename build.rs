use std::process::Command;

fn main() {
   let version = {
      let output = Command::new("git")
         .args(&["describe", "--always", "--dirty", "--tags"])
         .output()
         .unwrap();
      if output.stdout.is_empty() {
         "unknown".to_string()
      } else {
         String::from_utf8(output.stdout).unwrap()
      }
   };

   let hash = {
      let output = Command::new("git")
         .args(&["rev-parse", "HEAD"])
         .output()
         .unwrap();
      String::from_utf8(output.stdout).unwrap()
   };

   println!("cargo:rustc-env=GIT_VERSION={}", version);
   println!("cargo:rustc-env=GIT_HASH={}", hash);
}

// ex:expandtab sw=3 ts=3
