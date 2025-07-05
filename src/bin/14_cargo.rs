// we can install any package in rust from crates.io/crates using `cargo add <package_name>` command.

use chrono::{Local, Utc};
use rand::{Rng, thread_rng};

fn main() {
  let mut rng = thread_rng();
  let n: u32 = rng.r#gen();
  println!("Random Number:{}", n);

  get_time();
}

fn get_time() {
  let now = Utc::now();
  println!("current date and time in utc is: {}", now);

  let formatted = now.format("%Y-%m-%d %H:%M:%S");
  println!("formatted data and time: {}",formatted);

  let local=Local::now();
  println!("current date and time in local: {}",local);
}
