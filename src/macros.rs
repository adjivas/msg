// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/xsi
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `getpid` macro returns the PID of
/// program.

#[macro_export]
macro_rules! getpid {
  () => ({
    unsafe { xsi::ffi::getpid() }
  });
}

/// The `ftok` macro returns the System-V'IPC
/// key from pathname.

#[macro_export]
macro_rules! ftok {
  () => ({
    match unsafe {
      xsi::ffi::ftok(
        xsi::ffi::TOK_PATHNAME.as_ptr() as *mut i8,
        xsi::ffi::TOK_PROJ_ID as xsi::ffi::c_int,
      )
    } {
      -1 => None,
      key => Some(key as u64),
    }
  });
  ($pathname: expr) => ({
    match unsafe {
      xsi::ffi::ftok(
        $pathname.as_ptr() as *mut i8,
        xsi::ffi::TOK_PROJ_ID as xsi::ffi::c_int
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
      xsi::ffi::msgget(
        $key as xsi::ffi::c_int,
        xsi::ffi::Ipc::CREAT as xsi::ffi::c_int | 0o0666,
      )
    } {
        -1 => None,
        id => Some(id as i32),
    }
  });
  ($key: expr, $msgfl: expr) => ({
    match unsafe {
      msgget(
        $key as xsi::ffi::c_int,
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
        let aref = &*(p as *const [ffi::c_char; xsi::ffi::MSG_BUFF as usize]);

        p = p.offset(ffi::MSG_BUFF as isize);
        aref
      },
    });

    match unsafe {
      xsi::ffi::msgsnd(
        $id as xsi::ffi::c_int,
        buf,
        xsi::ffi::MSG_BUFF as xsi::ffi::size_t ,
        xsi::ffi::Ipc::NOWAIT as xsi::ffi::c_int,
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
    let from: xsi::ffi::pid_t = getpid!();

    msgrcv!($id, from)
  });
  ($id: expr, $from: expr) => ({
    let mut rcv = Box::new(ffi::MsgBuf {
      mtype: $from as xsi::ffi::c_long,
      mtext: [0 as xsi::ffi::c_char; xsi::ffi::MSG_BUFF as usize],
    });

    match unsafe {
      xsi::ffi::msgrcv(
        $id as xsi::ffi::c_int,
        &mut *rcv,
        xsi::ffi::MSG_BUFF as xsi::ffi::size_t ,
        $from as xsi::ffi::c_long,
        xsi::ffi::Ipc::NOWAIT as xsi::ffi::c_int,
      )
    } {
      -1 => None,
      _ => Some(rcv.mtext),
    }
  });
}
