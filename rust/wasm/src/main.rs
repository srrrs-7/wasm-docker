mod cache;

use crate::cache::cache::Cache;

fn main() {
  let mut cache_list = Cache::new();
  cache_list.set("key", "value");
  println!("{}", cache_list.get(&"key").unwrap());
}
