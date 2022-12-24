#!/bin/zsh

d=${0:a:h}
f=/usr/share/vim/vim90/doc/builtin.txt
if [ ! -f $f ];then
	exit
fi

if [ -z "$1" ];then
	exit
fi

case $OSTYPE in
	darwin*)
		a=`grep -Rn "*$1*" $f|cut -d : -f 2|head -n 1`
		;;
	linux*)
		a=`grep -Rn "*$1*" $f|cut -d : -f 1|head -n 1`
		;;
esac

b=$(($a + 500))
tmp=`awk "NR==$a,NR==$b" $f`
b=`echo "$tmp"|grep -n "^[a-z]"|awk "NR==2"|cut -d : -f 1|head -n 1`
echo "$tmp"|awk "NR==1,NR==$b"|sed -e '$d' 
