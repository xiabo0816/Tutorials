package main

/*
#cgo CFLAGS: -Icpp
#cgo LDFLAGS: -L. -lgotest
#include "cwrap.h"
*/
import "C"

func main() {
    C.call()
}