// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/xsi
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate xsi;
#[macro_use] extern crate sig;
#[macro_use] extern crate io;

fn receive (sig: i32) {
    println!("{} intercepted", sig);
}

fn main () {
    let from: i32 = getpid!();

    println!("{}", from);
    signal!(sig::ffi::Sig::USR1, receive);
    loop {
        if let Some(at) = read_number!() {
            kill!(at, sig::ffi::Sig::USR1);
        }
        if let Some((len, line)) = read!() {
            write!(&line, len);
        }
    }
}
