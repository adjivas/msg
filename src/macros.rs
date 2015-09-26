// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lem-ipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub const TOK_PROJ_ID: u32 = 0;
pub const MSG_BUFF: usize = 2;

/// The `getpid` macro returns the PID of
/// program.

#[macro_export]
macro_rules! getpid {
  () => ({
    unsafe { ffi::getpid() as i32 }
  });
}

/// The `write` macro writes the message and
/// returns None if fails or Some(0).

#[macro_export]
macro_rules! write {
  ($text: expr, $len: expr) => ({
    write!($text, $len, 1)
  });
  ($text: expr, $len: expr, $out: expr) => ({
    match unsafe {
      ffi::write($out, $text as *const c_char, $len)
    } {
      -1 => None,
      xsi => Some(xsi as i32),
    }
  });
}

/// The `ftok` macro returns the System-V'IPC
/// key from pathname.

#[macro_export]
macro_rules! ftok {
  () => ({
    match unsafe {
      ffi::ftok(
        b"/tmp".as_ptr() as *mut i8,
        macros::TOK_PROJ_ID as c_int,
      )
    } {
      -1 => None,
      key => Some(key as u64),
    }
  });
  ($pathname: expr) => ({
    match unsafe {
      ffi::ftok(
        $pathname.as_ptr() as *mut i8,
        macros::TOK_PROJ_ID as c_int
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
    match unsafe {
      ffi::msgget(
        $key as c_int,
        ffi::Ipc::CREAT as c_int | 0o0666,
      )
    } {
        -1 => None,
        id => Some(id as i32),
    }
  });
  ($key: expr, $msgfl: expr) => ({
    match unsafe {
      msgget(
        $key as c_int,
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
    let mut p = $text.as_ptr();
    let buf = &mut *Box::new(ffi::MsgBuf {
      mtype: $at as i64,
      mtext: *unsafe {
        let aref = &*(p as *const [c_char; macros::MSG_BUFF as usize]);

        p = p.offset(macros::MSG_BUFF as isize);
        aref
      },
    });

    match unsafe {
      ffi::msgsnd(
        $id as c_int,
        buf,
        macros::MSG_BUFF as size_t,
        ffi::Ipc::NOWAIT as c_int,
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
  ($id: expr) => ({
    let from: pid_t = getpid!();

    msgrcv!($id, from)
  });
  ($id: expr, $from: expr) => ({
    let mut rcv = Box::new(ffi::MsgBuf {
      mtype: $from as c_long,
      mtext: [0 as c_char; macros::MSG_BUFF as usize],
    });

    match unsafe {
      ffi::msgrcv(
        $id as c_int,
        &mut *rcv,
        macros::MSG_BUFF as size_t,
        $from as c_long,
        ffi::Ipc::NOWAIT as c_int
      )
    } {
      -1 => None,
      _ => Some(rcv.mtext),
    }
  });
}
