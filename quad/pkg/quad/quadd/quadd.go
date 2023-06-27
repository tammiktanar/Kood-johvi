package quadd

func QuadD(x, y int) {
	println("Requested x:", x, " y:", y)
	for i := 0; i < y; i++ {
		for k := 0; k < x; k++ {
			if i == 0 || i == (y-1) {
				switch x - k {
				case 1:
					println("C")
				case x:
					print("A")
				default:
					print("B")
				}
			} else {
				switch x - k {
				case 1:
					println("B")
				case x:
					print("B")
				default:
					print(" ")
				}
			}
		}
	}
	println("")
}
