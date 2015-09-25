extern crate libc;

use libc::types::os::arch::posix88::{
  pid_t,
  ssize_t,
};

use libc::types::os::arch::c95::{
  c_int,
  c_long,
  c_char,
  size_t,
};

extern "C" {
  fn getpid() -> pid_t;
  fn ftok(path: *mut c_char, id: c_int) -> c_long;
  fn msgget(key: c_int, msgflg: c_int) -> c_int;
  fn msgsnd(id: c_int, snd: *mut MsgBuf, len: size_t, flag: c_int) -> c_int;
  fn msgrcv(id: c_int, snd: *mut MsgBuf, len: size_t, mtype: c_long, flag: c_int) -> ssize_t;
  fn write(fd: c_int, buf: *const c_char, len: size_t) -> ssize_t;
}

#[repr(C)]
struct MsgBuf {
  mtype: c_long,
  mtext: [c_char; Ipc::MSG_BUFF as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
enum Ipc {
  CREAT = 0o0001000,
  NOWAIT = 2048,
  TOK_PROJ_ID = 0,
  MSG_BUFF = 2,
}

macro_rules! getpid {
  () => ({
    unsafe { getpid() as i32 }
  });
}

macro_rules! write {
  ($text: expr, $len: expr) => ({
    write!($text, $len, 1)
  });
  ($text: expr, $len: expr, $out: expr) => ({
    match unsafe {
      write($out, $text as *const c_char, $len)
    } {
      -1 => None,
      xsi => Some(xsi as i32),
    }
  });
}

macro_rules! ftok {
  () => ({
    match unsafe {
      ftok(
        b"/tmp".as_ptr() as *mut i8,
        Ipc::TOK_PROJ_ID as c_int,
      )
    } {
      -1 => None,
      key => Some(key as u64),
    }
  });
  ($pathname: expr) => ({
    match unsafe {
      ftok(
        $pathname.as_ptr() as *mut i8,
        Ipc::TOK_PROJ_ID as c_int
      )
    } {
      -1 => None,
      key => Some(key as u64),
    }
  });
}

macro_rules! msgget {
  ($key: expr) => ({
    match unsafe {
      msgget(
        $key as c_int,
        Ipc::CREAT as c_int | 0o0666,
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

macro_rules! msgsnd {
  ($id: expr, $at: expr, $text: expr) => ({
    let mut p = $text.as_ptr();
    let buf = &mut *Box::new(MsgBuf {
      mtype: $at as i64,
      mtext: *unsafe {
        let aref = &*(p as *const [c_char; Ipc::MSG_BUFF as usize]);
        p = p.offset(Ipc::MSG_BUFF as isize);
        aref
      },
    });

    match unsafe {
      msgsnd(
        $id as c_int,
        buf,
        Ipc::MSG_BUFF as size_t,
        Ipc::NOWAIT as c_int,
      )
    } {
      -1 => None,
      xsi => Some(xsi as i32),
    }
  });
}

macro_rules! msgrcv {
  ($id: expr) => ({
    let from: pid_t = getpid!();

    msgrcv!($id, from)
  });
  ($id: expr, $from: expr) => ({
    let mut rcv = Box::new(MsgBuf {
      mtype: $from as c_long,
      mtext: [0 as c_char; Ipc::MSG_BUFF as usize],
    });

    match unsafe {
      msgrcv(
        $id as c_int,
        &mut *rcv,
        Ipc::MSG_BUFF as size_t,
        $from as c_long,
        Ipc::NOWAIT as c_int
      )
    } {
      -1 => None,
      _ => Some(rcv.mtext),
    }
  });
}

#[allow(unused_assignments)]
fn main() {
  let at:i32 = getpid!();
  let key:u64 = ftok!(b"/nfs/zfs-student-5/users/2013/adjivas").unwrap();
  let id:i32 = msgget!(key).unwrap();
  msgsnd!(id, at, b"a\n").unwrap();
  let msg = msgrcv!(id).unwrap();

  write!(&msg, 2);
}
