// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/xsi
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate xsi;
#[macro_use] extern crate sig;
#[macro_use] extern crate io;

fn receive (_: i32) {
    if let Some(key) = ftok!() {
        if let Some(id) = msgget!(key) {
            if let Some(line) = msgrcv!(id) {
                write!(&line, xsi::ffi::MSG_BUFF);
            }
        }
    }
}

fn main () {
    let from: i32 = getpid!();

    println!("{}", from);
    signal!(sig::ffi::Sig::USR1, receive);

    if let Some(key) = ftok!() {
        if let Some(id) = msgget!(key) {
            loop {
                if let Some(at) = read_number!() {
                    if let Some((_, line)) = read!() {
                        msgsnd!(id, at, &line);
                        kill!(at, sig::ffi::Sig::USR1);
                    }
                }
            }
        }
    }
}
