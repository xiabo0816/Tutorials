# 使用g++编译dll给go调用

```
Windows 10

go version go1.12 windows/amd64

g++ (tdm64-1) 5.1.0

Copyright (C) 2015 Free Software Foundation, Inc.

This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

```

> mingw32-make.exe

> go build

这里，不是go build xxx.go命令，go build xxx.go只会单独编译xxx.go文件，而不会自动编译链接相关的C源文件

if mac

> go build -ldflags -w
