#!/bin/zsh

d=${0:a:h}
dd=$HOME/.config/amx/vimrc
if [ ! -d $dd ];then
	mkdir -p $dd
fi

o=`curl -sL https://vim-jp.org/reading-vimrc |grep -e "https://github.com"|grep -e '\.vim' -e '\.lua'|cut -d '"' -f 2|sed 's#github.com#raw.githubusercontent.com#g'|sed 's#/blob##g' |tr '\n' '!'|sed 's#!# -O #g'`
cd $dd
curl -sL `echo $o`
