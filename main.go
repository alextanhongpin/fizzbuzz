package main

import "fmt"

func main() {
	fizzbuzz := 0
	fizz := 0
	buzz := 0

	for i := 1; i <= 100; i++ {
		if i%3 == 0 && i%5 == 0 {
			fmt.Println("FizzBuzz")
			fizzbuzz++
		} else if i%3 == 0 {
			fmt.Println("Fizz")
			fizz++
		} else if i%5 == 0 {
			fmt.Println("Buzz")
			buzz++
		}
	}

	fmt.Printf("found FizzBuzz = %d Fizz = %d Buzz = %d\n", fizzbuzz, fizz, buzz)
}
