package piscine

func Fibonacci(n int) int {
	if n < 0 {
		return -1
	} else {
		if n <= 1 {
			return n
		}
		return Fibonacci(n-1) + Fibonacci(n-2)
	}
}
