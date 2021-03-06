extern crate redis = "redis#0.1";

fn main() {
  let mut redis = redis::Client::new("127.0.0.1:6379");
  redis.set_int("counter", 1);
  redis.set("key", "Hello");

  redis.incr("counter");

  let counter = redis.get_int("counter").unwrap().unwrap();
  println!("counter = {}", counter);

  let key = redis.get_str("key").unwrap().unwrap();
  println!("key = {}", key);

  match redis.get("key").unwrap() {
    redis::Nil => {
      println!("Key not found")
    }
    redis::Data(ref s) => {
      println!("{:?}", std::str::from_utf8(*s))
    }
    _ => { fail!() }
  }
}
