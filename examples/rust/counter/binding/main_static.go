package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo CFLAGS: -std=c99
#cgo LDFLAGS: -L. -l:libcounterlib.so
#include "./counter.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	C.init_stuff()
	C.hello(C.CString("Counter Station"))
	counter_station := C.create_counter_station()

	init_msg := C.counter_station_init(counter_station)
	state := C.verify_and_get_commit_init(init_msg)
	fmt.Printf("Init State: %s\n", C.GoString(state))

	msg1 := C.counter_station_submit(counter_station)

	msg2 := C.counter_station_submit(counter_station)

	fmt.Printf("msg1: %T, %d\n", msg1, unsafe.Sizeof(msg1))
	fmt.Printf("msg2: %T, %d\n", msg2, unsafe.Sizeof(msg2))

	commit := C.verify_and_get_commit(msg1)
	fmt.Printf("Msg1 Commit: %s\n", C.GoString(commit))

	commit2 := C.verify_and_get_commit(msg2)
	fmt.Printf("Msg2 Commit: %s\n", C.GoString(commit2))
}
