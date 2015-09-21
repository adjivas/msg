extern "C" {
  fn getpid() -> i64;
  fn ftok(path: *mut i8, id: u32) -> i64;
  fn msgget(key: i64, msgflg: u32) -> i64;
  fn msgsnd(id: i64, snd: *mut MsgBuf, len: u32, flag: i32) -> i8;
  fn msgrcv(id: i64, snd: *mut MsgBuf, len: u32, mtype: i64, flag: i32) -> i64;
}

#[repr(C)]
struct MsgBuf {
  mtype: i64,
  mtext: [i8; Ipc::MSG_BUFF as usize],
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
    unsafe { getpid() }
  })
}

macro_rules! ftok {
  () => ({
    match unsafe {
      ftok(b".".as_ptr() as *mut i8, Ipc::TOK_PROJ_ID as u32)
    } {
      -1 => None,
      key => Some(key),
    }
  });
  ($pathname: expr) => ({
    match unsafe {
      ftok($pathname.as_ptr() as *mut i8, Ipc::TOK_PROJ_ID as u32)
    } {
      -1 => None,
      key => Some(key),
    }
  })
}

macro_rules! msgget {
  ($key: expr) => ({
    match unsafe {
      msgget($key, Ipc::CREAT as u32 | 0o066)
    } {
      -1 => None,
      id => Some(id),
    }
  });
  ($key: expr, $msgfl: expr) => ({
    match unsafe {
      msgget($key, $msgfl)
    } {
      -1 => None,
      id => Some(id),
    }
  })
}

macro_rules! msgsnd {
  ($id: expr, $at: expr) => ({
    match unsafe { msgsnd(
        $id,
        &mut *Box::new(MsgBuf {
          mtype: $at,
          mtext: ['a' as i8, '\n' as i8],
        }),
        Ipc::MSG_BUFF as u32,
        Ipc::NOWAIT as i32
      )
    } {
      -1 => None,
      xsi => Some(xsi),
    }
  })
}

macro_rules! msgrcv {
  ($id: expr) => ({
    let from: i64 = getpid!();

    msgrcv!($id, from)
  });
  ($id: expr, $from: expr) => ({
    let mut rcv = Box::new(MsgBuf {
      mtype: $from,
      mtext: [0i8, 0i8],
    });

    match unsafe {
      msgrcv($id, &mut *rcv, Ipc::MSG_BUFF as u32, $from, Ipc::NOWAIT as i32)
    } {
      -1 => None,
      _ => Some(rcv.mtext),
    }
  });

}

fn main() {
  let at:i64 = getpid!();
  let key:i64 = ftok!().unwrap();
  let id:i64 = msgget!(key).unwrap();

  msgsnd!(id, at).unwrap();
  println!("{:?}", msgrcv!(id).unwrap() );
}
