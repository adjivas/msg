// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/msg
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `ftok` macro returns the System-V'IPC
/// key from pathname.

#[macro_export]
macro_rules! ftok {
    () => ({
        extern crate msg;
        match unsafe {
          msg::ffi::ftok (
            msg::ffi::TOK_PATHNAME.as_ptr() as *mut i8,
            msg::ffi::TOK_PROJ_ID as i32,
          )
        } {
            -1 => None,
            key => Some(key as u64),
        }
    });
    ($pathname: expr) => ({
        extern crate msg;
        match unsafe {
            msg::ffi::ftok (
                $pathname.as_ptr() as *mut i8,
                msg::ffi::TOK_PROJ_ID as i32
            )
        } {
            -1 => None,
            key => Some(key as u64),
        }
    });
}

/// The `msgget` macro returns identifiant of
/// XSI message queue.

#[macro_export]
macro_rules! msgget {
    ($key: expr) => ({
        extern crate msg;
        msgget! (
            $key,
            msg::ffi::Ipc::CREAT as i32 | 0o0666
        )
    });
    ($key: expr, $msgfl: expr) => ({
        extern crate msg;
        match unsafe {
          msg::ffi::msgget (
            $key as i32,
            $msgfl,
          )
        } {
            -1 => None,
            id => Some(id as i32),
        }
    });
}

/// The `msgsnd` macro sends a new message
/// to the XSI queue.

#[macro_export]
macro_rules! msgsnd {
    ($id: expr, $at: expr, $text: expr) => ({
        extern crate msg;
        let mut p = $text.as_ptr();
        let mut buf = msg::ffi::MsgBuf {
            mtype: $at as i64,
            mtext: *unsafe {
                let aref = &*(p as *const [i8; msg::ffi::MSG_BUFF as usize]);

                p = p.offset(msg::ffi::MSG_BUFF as isize);
                aref
            },
        };

        match unsafe {
            msg::ffi::msgsnd (
                $id as i32,
                &mut buf,
                msg::ffi::MSG_BUFF as u64,
                msg::ffi::Ipc::NOWAIT as i32,
            )
        } {
            -1 => None,
            xsi => Some(xsi as i32),
        }
    });
}

/// The `msgrcv` macro recuperates a new message
/// according to $from variable.

#[macro_export]
macro_rules! msgrcv {
    ($id: expr, $from: expr) => ({
        extern crate msg;
        let mut rcv = msg::ffi::MsgBuf {
            mtype: $from as i64,
            mtext: [0 as i8; msg::ffi::MSG_BUFF as usize],
        };

        match unsafe {
            msg::ffi::msgrcv (
                $id as i32,
                &mut rcv,
                msg::ffi::MSG_BUFF as u64 ,
                $from as i64,
                msg::ffi::Ipc::NOWAIT as i32,
            )
        } {
            -1 => None,
            _ => Some(rcv.mtext),
        }
    });
}

/// The `msgclr` macro returns a information
/// according the argument command.

#[macro_export]
macro_rules! msgctl {
    ($id: expr, $cmd: expr) => ({
        extern crate std;
        msgctl!($id, $cmd, std::ptr::null_mut())
    });
    ($id: expr, $cmd: expr, $info: expr) => ({
        extern crate msg;
        match unsafe {
            msg::ffi::msgctl (
                $id,
                $cmd as i32,
                $info
            )
        } {
            -1 => false,
            _ => true,
        }
    });
}
