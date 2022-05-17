package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo CFLAGS: -std=c99
#cgo LDFLAGS: -L. -l:libvotinglib.so
#include "./voting.h"
*/
import "C"
import "fmt"

func main() {
	C.init_stuff()
	C.hello(C.CString("John Smith"))
	polling_station := C.create_polling_station()

	init_msg := C.polling_station_init(polling_station)
	state := C.verify_and_get_commit_init(init_msg)
	fmt.Printf("Init State: %s\n", C.GoString(state))

	ballot_1 := C.vote(0, true)
	msg := C.polling_station_submit(polling_station, ballot_1)

	commit := C.verify_and_get_commit(msg)
	fmt.Printf("Commit: %s\n", C.GoString(commit))
}
