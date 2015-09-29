// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lem-ipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]

extern crate libc;

/// The `libc` type is the list of C types.

pub type pid_t = libc::types::os::arch::posix88::pid_t;
pub type ssize_t = libc::types::os::arch::posix88::ssize_t;
pub type c_int = libc::types::os::arch::c95::c_int;
pub type c_long = libc::types::os::arch::c95::c_long;
pub type c_char = libc::types::os::arch::c95::c_char;
pub type size_t = libc::types::os::arch::c95::size_t;

/// The `Ipc` enum is a POSIX Standard
/// for System V.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Ipc {
  CREAT = 0o0001000, // POSIX
  NOWAIT = 2048, // POSIX
}

/// The `TOK_*, MSG_BUFF` const are default values
/// for macros.

#[allow(dead_code)]
pub const TOK_PATHNAME: &'static [u8; 4] = b"/tmp";
pub const TOK_PROJ_ID: u32 = 0;
pub const MSG_BUFF: usize = 1024;

/// The `C` extern is list of libc functions required
/// by the project.

#[cfg(any(unix))]
extern "C" {
  pub fn getpid() -> pid_t;
  pub fn ftok(path: *mut c_char, id: c_int) -> c_long;
  pub fn msgget(key: c_int, msgflg: c_int) -> c_int;
  pub fn msgsnd(id: c_int, snd: *mut MsgBuf, len: size_t, flag: c_int) -> c_int;
  pub fn msgrcv(id: c_int, snd: *mut MsgBuf, len: size_t, mtype: c_long, flag: c_int) -> ssize_t;
  pub fn write(fd: c_int, buf: *const c_char, len: size_t) -> ssize_t;
}

/// The MsgBuf struct is a structure required
/// by `msgsnd` and `msgrcv` functions.

#[repr(C)]
pub struct MsgBuf {
  pub mtype: c_long,
  pub mtext: [c_char; MSG_BUFF as usize],
}
