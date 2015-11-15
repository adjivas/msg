// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate msg;

#[allow(unused_unsafe)]
#[allow(unused_assignments)]
#[test]
fn the_message () {
    let id: i32 = msgget! (
        ftok!().expect("ftok! fail")
    ).expect("msgget! fail");

    assert_eq!(msgsnd!(id, 42, "hello\n").is_some(), true);
    assert_eq!(msgrcv!(id, 42).is_some(), true);
    assert_eq!(msgctl!(id, msg::ffi::Ipc::RMID), true);
}
