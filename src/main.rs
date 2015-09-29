// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lem-ipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;
extern crate readline;

#[macro_use]
mod macros;
mod ffi;

/// The `Msg` structure contains the access identifiant to
/// the XSI interface.

pub struct Msg {
  id: i32,
}

#[allow(unused_assignments)]
impl Msg {

  /// The `new` constructor function returns a interface to
  /// a XSI message queue.

  pub fn new (
    key: u64
  ) -> Self {
    Msg {
      id: msgget!(key).unwrap()
    }
  }

  /// The `send` function adds/pushs a message in the XSI queue
  /// at a receiver.

  pub fn send (
    &self,
    text: [ffi::c_char; ffi::MSG_BUFF],
    at: i32
  ) -> Option<i32> {
    msgsnd!(self.id, at, text)
  }

  /// The `receive` function gets/takes one message
  /// from the XSI queue.

  pub fn receive (&self) -> Option<[i8; ffi::MSG_BUFF]> {
    msgrcv!(self.id)
  }

  /// The `receive` function gets/takes and prints one message
  /// from the XSI queue.

  pub fn display_receive (&self) -> Option<i32> {
    match self.receive() {
      Some(text) => write!(&text, ffi::MSG_BUFF),
      None => None,
    }
  }
}

/// The `Lem` structure contains the init key to
/// the msg struct.

#[allow(dead_code)]
pub struct Lem {
  key: u64,
  msg: Msg,
}

impl Lem {

  /// The `new` constructor function returns
  // the principal structure.

  pub fn new () -> Self {
    let key: u64 = ftok!().unwrap();

    Lem {
      key: key,
      msg: Msg::new(key),
    }
  }
}

fn main() {
  let lem: Lem = Lem::new();
  let at: i32 = getpid!();

  loop {
    let (_, text):(i64, [readline::ffi::c_char; readline::ffi::BUFF]) = readline::sentence().unwrap();

    lem.msg.send(text, at).unwrap();
    lem.msg.display_receive();
  }
}
