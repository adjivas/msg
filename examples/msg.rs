// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/msg
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate msg;
#[macro_use] extern crate io_synesthesist;

#[allow(unused_assignments)]
fn main () {
    if let Some(key) = ftok!() {
        if let Some(id) = msgget!(key) {
            msgsnd!(id, 42, "hello\n");
            if let Some(line) = msgrcv!(id, 42) {
                write!(&line, msg::ffi::MSG_BUFF);
            }
        }
    }
}
