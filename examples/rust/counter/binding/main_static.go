package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo CFLAGS: -std=c99
#cgo LDFLAGS: -L. -l:libcounterlib.so
#include <stdlib.h>
#include "./counter.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

type RISC0UnlockBlock struct {
	message *C.struct_SubmitCounterMessage
}

func (b *RISC0UnlockBlock) Verify() *C.char {
	return C.verify_and_get_commit(b.message)
}

func main() {

	C.hello(C.CString("Counter Station"))
	counter_station := C.create_counter_station()

	// Construct a RISC0UnlockBlock
	init_msg := C.counter_station_init(counter_station)

	state := C.verify_and_get_commit_init(init_msg)
	fmt.Printf("Init State: %s\n", C.GoString(state))

	msg1 := C.counter_station_submit(counter_station)
	block := &RISC0UnlockBlock{
		message: msg1,
	}
	commit := block.Verify()
	fmt.Printf("RISC0UnlockBlock Commit 0: %s\n", C.GoString(commit))
	C.free(unsafe.Pointer(commit))

	msg2 := C.counter_station_submit(counter_station)
	block.message = msg2
	commit = block.Verify()
	fmt.Printf("RISC0UnlockBlock Commit 1: %s\n", C.GoString(commit))
	C.free(unsafe.Pointer(commit))
}
