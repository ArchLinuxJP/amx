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
# timeline(sync)
$ amx t
$ amx t -j '#example:matrix.org'

# start bot(sync)
$ amx t --type test

# message post
$ amx p "hello world" -j '#example:matrix.org'
$ amx p "hello world" -i '!example:matrix.org'

# message post : save room
$ amx p "hello world" -r '!example:matrix.org'
$ amx p "2"

# join room
$ amx r -j '#example:matrix.org'

# joined room info
$ amx r

# change account
$ amx a -d
$ amx a -s
```

## config

> ~/.config/amx/config.toml

```sh
home_server = ""
access_token = ""
username = ""
password = ""
room_id = ""
```

## token

```sh
# token
$ curl -XGET "https://localhost:8448/_matrix/client/r0/login"
$ curl -XPOST -d '{"type":"m.login.password", "user":"example", "password":"wordpass"}' "https://localhost:8448/_matrix/client/r0/login"
```

## vimrc

https://vim-jp.org/reading-vimrc

```sh
# bot account
$ vim dendrite.yaml
- registration_disabled: true
- enable_registration_captcha: false
+ registration_disabled: false
+ enable_registration_captcha: true

$ ssh dendrite
# https://matrix-org.github.io/dendrite/administration/createusers
$ /usr/bin/create-account -config /etc/dendrite/dendrite.yaml -username xxx -password xxx
```

```sh
# start
$ rm -rf ~/.config/amx/vimrc
$ cp vimrc.zsh ~/.config/amx/
$ vimrc.zsh

$ amx a -s
$ amx r -j '#vim-jp_reading-vimrc:gitter.im'
$ amx t --type vimrc

# check user
$ amx t --type vimrc -u @syui:syui.cf
```

fcron : 日時: 毎週土曜日夜23時(JST)

### ref

https://docs.rs/matrix-sdk/latest

https://docs.rs/ruma/latest/ruma

https://github.com/matrix-org/matrix-rust-sdk

https://github.com/ruma/ruma

https://github.com/ksk001100/seahorse
