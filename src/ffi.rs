// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/msg
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

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
  pub fn ftok(path: *mut i8, id: i32) -> i64;
  pub fn msgget(key: i32, msgflg: i32) -> i32;
  pub fn msgsnd(id: i32, snd: *mut MsgBuf, len: u64,
                flag: i32) -> i32;
  pub fn msgrcv(id: i32, snd: *mut MsgBuf, len: u64, mtype: i64,
                flag: i32) -> i64;
}

/// The MsgBuf struct is a structure required
/// by `msgsnd` and `msgrcv` functions.

#[repr(C)]
pub struct MsgBuf {
  pub mtype: i64,
  pub mtext: [i8; MSG_BUFF as usize],
}
