const _TOK_PROJ_ID:u32 = 0;
const _MSG_RIGHT:u32 = 0666 | 512;
const _MSG_BUFF:u32 = 2;
const _MSG_TYPE:i32 = 42;
const _MSG_IPC_NOWAIT: i32 = 2048;


extern "C" {
  fn ftok(path: *mut i8, id: u32) -> i32;
  fn msgget(key: i32, msgflg: u32) -> i32;
  fn msgsnd(id: i32, snd: *mut MsgBuf, len: u32, flag: i32) -> i32;
  fn msgrcv(id: i32, snd: *mut MsgBuf, len: u32, mtype: i32, flag: i32) -> i32;
}

#[repr(C)]

struct MsgBuf {
  mtype: i32,
  mtext: [i8; 2],
}

fn main() {
  let key:i32 = unsafe { ftok(b".".as_ptr() as *mut i8, _TOK_PROJ_ID) };
  let id:i32 = unsafe { msgget(key, _MSG_RIGHT) };

  println!("key:{}", key);
  println!("id:{}", id);
  let mut snd = Box::new(MsgBuf {
    mtype: _MSG_TYPE,
    mtext: ['a' as i8, 'z' as i8],
  });
  println!("work: {}", unsafe { msgsnd(id, &mut *snd, _MSG_BUFF, _MSG_IPC_NOWAIT) });

  let mut rcv = Box::new(MsgBuf {
    mtype: 42i32,
    mtext: [0i8, 0i8],
  });
  println!("fail: {}", unsafe { msgrcv(id, &mut *rcv, _MSG_BUFF, _MSG_TYPE, _MSG_IPC_NOWAIT) });
  println!("bad result:{:?}", rcv.mtext);
}
