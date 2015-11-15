var searchIndex = {};
searchIndex['msg'] = {"items":[[0,"","msg","",null,null],[0,"ffi","","",null,null],[3,"MsgBuf","msg::ffi","The `MsgBuf` struct is a structure required\nby `msgsnd` and `msgrcv` functions.",null,null],[12,"mtype","","",0,null],[12,"mtext","","",0,null],[3,"MsqidDs","","The `MsqidDs` struct is a structure required\nby `msgsnd` and `msgrcv` functions.",null,null],[3,"IpcPerm","","",null,null],[12,"uid","","",1,null],[12,"gid","","",1,null],[12,"cuid","","",1,null],[12,"cgid","","",1,null],[12,"mode","","",1,null],[12,"seq","","",1,null],[12,"key","","",1,null],[4,"Ipc","","The `Ipc` enum is a POSIX Standard\nfor System V.",null,null],[13,"CREAT","","",2,null],[13,"NOWAIT","","",2,null],[13,"EXCL","","",2,null],[13,"RMID","","",2,null],[13,"SET","","",2,null],[13,"STAT","","",2,null],[13,"INFO","","",2,null],[5,"ftok","","",null,null],[5,"msgget","","",null,null],[5,"msgsnd","","",null,null],[5,"msgrcv","","",null,null],[5,"msgctl","","",null,null],[17,"TOK_PATHNAME","","The `TOK_*, MSG_BUFF` const are default values\nfor macros.",null,null],[17,"TOK_PROJ_ID","","",null,null],[17,"MSG_BUFF","","",null,null],[11,"clone","","",2,{"inputs":[{"name":"ipc"}],"output":{"name":"ipc"}}],[14,"ftok!","msg","The `ftok` macro returns the System-V'IPC\nkey from pathname.",null,null],[14,"msgget!","","The `msgget` macro returns identifiant of\nXSI message queue.",null,null],[14,"msgsnd!","","The `msgsnd` macro sends a new message\nto the XSI queue.",null,null],[14,"msgrcv!","","The `msgrcv` macro recuperates a new message\naccording to $from variable.",null,null],[14,"msgctl!","","The `msgclr` macro returns a information\naccording the argument command.",null,null]],"paths":[[3,"MsgBuf"],[3,"IpcPerm"],[4,"Ipc"]]};
searchIndex['io'] = {"items":[[0,"","io","",null,null],[0,"ffi","","",null,null],[3,"Termios","io::ffi","",null,null],[12,"c_iflag","","",0,null],[12,"c_oflag","","",0,null],[12,"c_cflag","","",0,null],[12,"c_lflag","","",0,null],[12,"c_line","","",0,null],[12,"c_cc","","",0,null],[12,"c_ispeed","","",0,null],[12,"c_ospeed","","",0,null],[4,"ControlCharacter","","All the `const *` are default values\nof c_cc.",null,null],[13,"VINTR","","",1,null],[13,"VQUIT","","",1,null],[13,"VERASE","","",1,null],[13,"VKILL","","",1,null],[13,"VEOF","","",1,null],[13,"VTIME","","",1,null],[13,"VMIN","","",1,null],[13,"VSWTC","","",1,null],[13,"VSTART","","",1,null],[13,"VSTOP","","",1,null],[13,"VSUSP","","",1,null],[13,"VEOL","","",1,null],[13,"VREPRINT","","",1,null],[13,"VDISCARD","","",1,null],[13,"VWERASE","","",1,null],[13,"VLNEXT","","",1,null],[13,"VEOL2","","",1,null],[4,"InputModes","","All the `const *` are default values\nof c_iflag.",null,null],[13,"IGNBRK","","",2,null],[13,"BRKINT","","",2,null],[13,"IGNPAR","","",2,null],[13,"PARMRK","","",2,null],[13,"INPCK","","",2,null],[13,"ISTRIP","","",2,null],[13,"INLCR","","",2,null],[13,"IGNCR","","",2,null],[13,"ICRNL","","",2,null],[13,"IUCLC","","",2,null],[13,"IXON","","",2,null],[13,"IXANY","","",2,null],[13,"IXOFF","","",2,null],[13,"IMAXBEL","","",2,null],[13,"IUTF8","","",2,null],[4,"OutputModes","","All the `const *` are default values\nof c_oflag.",null,null],[13,"OPOST","","",3,null],[13,"OLCUC","","",3,null],[13,"ONLCR","","",3,null],[13,"OCRNL","","",3,null],[13,"ONOCR","","",3,null],[13,"ONLRET","","",3,null],[13,"OFILL","","",3,null],[13,"OFDEL","","",3,null],[13,"VT0","","",3,null],[13,"VT1","","",3,null],[4,"Speed","","All the `const *` are default values\nof c_cflag.",null,null],[13,"B0","","",4,null],[13,"B50","","",4,null],[13,"B75","","",4,null],[13,"B110","","",4,null],[13,"B134","","",4,null],[13,"B150","","",4,null],[13,"B200","","",4,null],[13,"B300","","",4,null],[13,"B600","","",4,null],[13,"B1200","","",4,null],[13,"B1800","","",4,null],[13,"B2400","","",4,null],[13,"B4800","","",4,null],[13,"B9600","","",4,null],[13,"B19200","","",4,null],[13,"B38400","","",4,null],[13,"B57600","","",4,null],[13,"B115200","","",4,null],[13,"B230400","","",4,null],[13,"B460800","","",4,null],[13,"B500000","","",4,null],[13,"B576000","","",4,null],[13,"B921600","","",4,null],[13,"B1000000","","",4,null],[13,"B1152000","","",4,null],[13,"B1500000","","",4,null],[13,"B2000000","","",4,null],[13,"B2500000","","",4,null],[13,"B3000000","","",4,null],[13,"B3500000","","",4,null],[13,"B4000000","","",4,null],[4,"LocalModes","","All the `const *` are default values\nof c_lflag.",null,null],[13,"ECHO","","",5,null],[13,"ECHOE","","",5,null],[13,"ECHOK","","",5,null],[13,"ECHONL","","",5,null],[13,"NOFLSH","","",5,null],[13,"TOSTOP","","",5,null],[13,"ISIG","","",5,null],[13,"ICANON","","",5,null],[4,"TermiosControl","","",null,null],[13,"GETS","","",6,null],[13,"SETS","","",6,null],[13,"SETSW","","",6,null],[13,"SETSF","","",6,null],[13,"GETA","","",6,null],[13,"SETA","","",6,null],[13,"SETAW","","",6,null],[13,"SETAF","","",6,null],[13,"SBRK","","",6,null],[13,"XONC","","",6,null],[13,"FLSH","","",6,null],[4,"Seek","","",null,null],[13,"SET","","",7,null],[13,"CUR","","",7,null],[13,"END","","",7,null],[5,"write","","",null,null],[5,"lseek","","",null,null],[5,"read","","",null,null],[5,"ioctl","","",null,null],[17,"NCCS","","The `NCCS` const is the default number of character\nparsed by the password input.",null,null],[17,"BUFF","","The `BUFF` and `STDIN_FILENO` const\nare default values for macros.",null,null],[17,"BUFF_READ_NUMBER","","",null,null],[17,"BUFF_READ_COMMAND","","",null,null],[17,"STDIN_FILENO","","",null,null],[11,"clone","","",0,{"inputs":[{"name":"termios"}],"output":{"name":"termios"}}],[11,"clone","","",1,{"inputs":[{"name":"controlcharacter"}],"output":{"name":"controlcharacter"}}],[11,"clone","","",2,{"inputs":[{"name":"inputmodes"}],"output":{"name":"inputmodes"}}],[11,"clone","","",3,{"inputs":[{"name":"outputmodes"}],"output":{"name":"outputmodes"}}],[11,"clone","","",4,{"inputs":[{"name":"speed"}],"output":{"name":"speed"}}],[11,"clone","","",5,{"inputs":[{"name":"localmodes"}],"output":{"name":"localmodes"}}],[11,"clone","","",6,{"inputs":[{"name":"termioscontrol"}],"output":{"name":"termioscontrol"}}],[11,"clone","","",7,{"inputs":[{"name":"seek"}],"output":{"name":"seek"}}],[14,"write!","io","The `write` macro writes to output the text\nand returns the Some 0i32 or None according to success.",null,null],[14,"writeln!","","The `writeln` macro writes to output the text with a breakline\nand returns the Some 0i32 or None according to success.",null,null],[14,"write_number!","","The `write_number` macro writes to output the number\nand returns the Some 0i32 or None according to success.",null,null],[14,"writeln_number!","","The `writeln_number` macro writes to output the number with a breakline\nand returns the Some 0i32 or None according to success.",null,null],[14,"write_character!","","The `write_character` macro writes to output the character\nand returns the Some 0i32 or None according to success.",null,null],[14,"writeln_character!","","The `write_character` macro writes to output the character with a breakline\nand returns the Some 0i32 or None according to success.",null,null],[14,"write_err!","","The `write_error` macro writes to output the error\nand returns the Some 0i32 or None according to success.",null,null],[14,"writeln_err!","","The `writeln_error` macro writes to output the error\nand returns the Some 0i32 or None according to success.",null,null],[14,"read!","","The `read` macro reads the input and returns None\nor the Some of thetuple (len, text).",null,null],[14,"read_character!","","The `read_character` macro reads and\nreturns one character.",null,null],[14,"read_number!","","The `read_number` macro reads and\nreturns the number.",null,null],[14,"read_command!","","The `read_command` macro reads and\nreturns the concat of all letter.",null,null],[14,"ioctl!","","The `ioctl` macro reads the input and\nreturns None or a tuple (len, text).",null,null]],"paths":[[3,"Termios"],[4,"ControlCharacter"],[4,"InputModes"],[4,"OutputModes"],[4,"Speed"],[4,"LocalModes"],[4,"TermiosControl"],[4,"Seek"]]};
initSearch(searchIndex);