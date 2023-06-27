package quade

func QuadE(x,y int) {
	firstA := true
	println("Requested x:", x, " y:", y)
	for i := 0; i < y; i++{
		for k := 0; k < x; k++{
			if i == 0 || i == (y-1){
				if firstA {
					if x == 1 {
						println("A")
					} else {
						print("A")
					}
					firstA = false
				} else {
					switch x - k {
						case 1:
							if y-1 == i && x != 1 && y != 1{
								println("A")
							} else {
								println("C")
							}
						case x:
							print("C")
						default:
							print("B")
					}
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