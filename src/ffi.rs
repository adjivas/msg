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
  CREAT = 0o0001000,
  NOWAIT = 2048,
  EXCL   = 0o0002000,
  RMID   = 0o0000000,
  SET    = 0o0000001,
  STAT   = 0o0000002,
  INFO   = 0o0000003,
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
    pub fn msgctl(id: i32, cmd: i32, info: *mut MsqidDs) -> i32;
}

/// The `MsgBuf` struct is a structure required
/// by `msgsnd` and `msgrcv` functions.

#[repr(C)]
pub struct MsgBuf {
  pub mtype: i64,
  pub mtext: [i8; MSG_BUFF as usize],
}

/// The `MsqidDs` struct is a structure required
/// by `msgsnd` and `msgrcv` functions.

#[repr(C)]
pub struct MsqidDs {
    msg_perm: IpcPerm, // structure describing operation permission.
    msg_stime: i64, // time of last msgsnd command.
    msg_rtime: i64, // time of last msgrcv command.
    msg_ctime: i64, // time of last change.
    msg_qnum: u64, // number of messages currently on queue.
    msg_qbytes: u64, // max number of bytes allowed on queue.
    msg_lspid: i32, // pid of last msgsnd().
    msg_lrpid: i32, // pid of last msgrcv().
}



#[repr(C)]
pub struct IpcPerm {
   pub uid: i64, // owner's user id.
   pub gid: i64, // owner's group id.
   pub cuid: i64, // creator's user id.
   pub cgid: i64, // creator's group id.
   pub mode: u16, // access modes.
   pub seq: u16, // slot usage sequence number.
   pub key: i64, // key.
}
