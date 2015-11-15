// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/msg
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate msg;

fn main () {
  if let Some(key) = ftok!() {
    if let Some(id) = msgget!(key) {
        println!("work: {}", msgctl!(id, msg::ffi::Ipc::RMID));
    }
  }
}
