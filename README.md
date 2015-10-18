# Msg-XSI

[![Build Status](https://travis-ci.org/adjivas/msg.svg)](https://travis-ci.org/adjivas/msg)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

#### How to build:
```shell
git clone https://github.com/adjivas/msg.git msgxsi && cd msgxsi
cargo build
```

#### How to use:
```shell
cargo run
1
ping
2 pong
```
```shell
cargo run
2
1 ping
pong
```

#### Cargo'git-Dependencies:
```shell
     Sig IO
       \ /
       Msg
```

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
\__ src
    |__ ffi.rs
    |__ lib.rs
    |__ macros.rs
    \__ main.rs
```

#### License:
*msg*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/msg/blob/master/LICENSE).
