package quada

func QuadA(x,y int) {
	println("Requested x:", x, " y:", y)
	for i := 0; i < y; i++{
		for k := 0; k < x; k++{
			if i == 0 || i == (y-1){
				switch x - k {
					case 1:
						println("o")
					case x:
						print("o")
					default:
						print("-")
				}
			} else {
				switch x - k {
					case 1:
						println("|")
					case x:
						print("|")
					default:
						print(" ")
				}
			}
		}
	}
	println("")
}