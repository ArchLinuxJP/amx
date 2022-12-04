`amx` is matrix client in rust.

## install

```sh
$ cargo build

$ mkdir -p ~/.config/amx
$ vim ~/.config/amx/config.toml
home_server = "https://matrix.archlinux.jp"
username = "username"
password = "password"

$ ./target/debug/amx
```

```sh
$ export PATH=$PATH:$HOME/.cargo/bin
$ cp -rf ./target/debug/amx ~/.cargo/bin
$ which amx
```

## config

> ~/.config/amx/config.toml

```sh
home_server = "https://matrix.archlinux.jp"
username = "$USER"
password = "$PASS"
```

## use

```sh
# join room
$ amx r -j '#ArchLinuxJP_general:gitter.im'
# join private room
$ amx r -i '!xxx:matrix.org'

# joined room info
$ amx r

# post message
$ amx p "hello world" -j '#example:matrix.org'
# private message
$ amx p "hello world" -i '!xxx:matrix.org'

# timeline
$ amx t
```

## token

```sh
# token
$ curl -XGET "https://localhost:8448/_matrix/client/r0/login"
$ curl -XPOST -d '{"type":"m.login.password", "user":"example", "password":"wordpass"}' "https://localhost:8448/_matrix/client/r0/login"
```

### ref

https://docs.rs/matrix-sdk/latest

https://docs.rs/ruma/latest/ruma

https://github.com/matrix-org/matrix-rust-sdk

https://github.com/ruma/ruma

https://github.com/ksk001100/seahorse
