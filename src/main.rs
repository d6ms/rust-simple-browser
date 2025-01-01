#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use noli::prelude::*;
use net_wasabi::http::HttpClient;

fn main() -> u64 {
  let client = HttpClient::new();
  // Make sure you run `python3 -m http.server 8000` at the project root
  match client.get("host.test".to_string(), 8000, "/test.html".to_string()) {
    Ok(res) => {
      print!("response:\n{:#?}", res);
    }
    Err(e) => {
      print!("error:\n{:#?}", e);
    }
  }
  0
}

entry_point!(main);
