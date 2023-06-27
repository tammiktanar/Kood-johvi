package UltimateDivMod

func UltimateDivMod(a *int, b *int) {
	var intA int = *a
	var intB int = *b
	*a = intA / intB
	*b = intA % intB
}
