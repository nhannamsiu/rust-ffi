package main

import "fmt"
/*
#cgo LDFLAGS: -L./../target/debug -lrustffi
#include "./../target/rustffi.h"
*/
import "C"

func main() {
    fmt.Println("Create expertise and assign to person--------------------------")
    expertise := C.expertise_create(C.CString("web3"), 15)
    person := C.person_create(C.CString("nam"), expertise)
    C.person_print(person)

    fmt.Println("Update expertise fields--------------------------")
    C.expertise_update(expertise, C.CString("DeFi"), 17)
    C.person_print(person)

    fmt.Println("Destroy person--------------------------")
    C.person_destroy(person)
    C.person_print(person)
    C.expertise_print(expertise)

    fmt.Println("Destroy expertise--------------------------")
    C.expertise_destroy(expertise)
    C.expertise_print(expertise)
}