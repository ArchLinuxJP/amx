`amx` is matrix client in rust.

## install

```sh
$ git clone https://git.syui.cf/syui/amx
$ cd amx
$ cargo build

$ mkdir -p ~/.config/amx
$ vim ~/.config/amx/config.toml
home_server = "https://matrix.org"
username = "username"
password = "password"

# path
$ echo $PATH
~/.cargo/bin
$ mv ./target/debug/amx ~/.cargo/bin/
$ amx
```

## use

```sh
# timeline
$ amx t
$ amx t -bot

# post message
$ amx p "hello world." -j '!example:matrix.org'

# room join
$ amx r -j '!example:matrix.org'
```

## config

> ~/.config/amx/config.toml

```sh
home_server = ""
access_token = ""
username = ""
password = ""
room_alias = ""
room_id = ""
```

## token

```sh
# token
$ curl -XGET "https://localhost:8448/_matrix/client/r0/login"
$ curl -XPOST -d '{"type":"m.login.password", "user":"example", "password":"wordpass"}' "https://localhost:8448/_matrix/client/r0/login"
```

ref : 

https://docs.rs/matrix-sdk/latest

https://docs.rs/ruma/latest/ruma

https://github.com/matrix-org/matrix-rust-sdk

https://github.com/ruma/ruma

https://github.com/ksk001100/seahorse
