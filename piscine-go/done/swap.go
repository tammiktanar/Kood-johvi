package Swap

func Swap(a *int, b *int) {
	var intA int = *a
	var intB int = *b

	*a = intB
	*b = intA
}
