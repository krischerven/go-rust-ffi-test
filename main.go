package main

import (
	"fmt"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
	"time"
)

// void nothing() {
// }
import (
	"C"
)

var (
	printer = message.NewPrinter(language.English)
)

func main() {
	measure(func() {
		// warmup
	}, 10_000_000)
	t0 := measure(func() {
		// nothing here
	}, 10_000_000)
	measure(func() {
		// warmup
	}, 10_000_000)
	t1 := measure(func() {
		C.nothing()
	}, 10_000_000)
	fmt.Println("go time (ms):", t0)
	fmt.Println("cgo time (ms):", t1)
	fmt.Println("cgo overhead:", fmt.Sprintf("%.1fx", t1/t0))
	fmt.Println("cgo cost/call (nanoseconds):",
		printer.Sprintf(
			"%.1f",
			((t1-t0)/10_000_000)*1_000_000,
		),
	)
	fmt.Println("cgo calls/second:", printer.Sprintf("%.0f", (1000/t1)*10_000_000))
	fmt.Println("go calls/second:", printer.Sprintf("%.0f", (1000/t0)*10_000_000))
}

func measure(f func(), times uint) float64 {
	t1 := time.Now()
	for i := uint(0); i < times; i++ {
		f()
	}
	return float64(time.Now().Sub(t1).Milliseconds())
}
