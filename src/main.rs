// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lem-ipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

#[macro_use]
mod macros;
mod ffi;

use libc::{
  pid_t,
  c_int,
  c_long,
  c_char,
  size_t,
};

#[allow(unused_assignments)]
fn main() {
  let at:i32 = getpid!();
  let key:u64 = ftok!(b"/nfs/zfs-student-5/users/2013/adjivas").unwrap();
  let id:i32 = msgget!(key).unwrap();
  msgsnd!(id, at, b"a\n").unwrap();
  let msg = msgrcv!(id).unwrap();

  write!(&msg, 2);
}
