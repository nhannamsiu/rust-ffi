package main

import (
    "fmt"
 )

/*
#cgo LDFLAGS: -L./../target/debug -lrustffi
#include "./../target/rustffi.h"
*/
import "C"

func main() {
    expertise := C.expertise_create(C.CString("web3"), 15)
    fmt.Println(expertise)
    C.expertise_print(expertise)
}