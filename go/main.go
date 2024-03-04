package main

/*
#cgo LDFLAGS: -L./../target/debug -lrustffi
#include "./../target/rustffi.h"
*/
import "C"

func main() {
    expertise := C.expertise_create(C.CString("web3"), 15)
    C.expertise_print(expertise)
    C.expertise_destroy(expertise)
    C.expertise_print(expertise)

    person := C.person_create(C.CString("nam"), expertise)
    C.person_print(person)
    C.person_destroy(person)
    C.person_print(person)
}