package quadb

/*Write a function QuadB that prints a valid rectangle of width x and of height y.
The function must draw the rectangles as in the examples.
x and y will always be positive numbers. Otherwise, the function should print nothing.*/

func QuadB(x,y int) {
	// if checks that the input values are not negative
	println("Requested x:", x, " y:", y)
	if x <= 0 || y <= 0 {
		} else {
		for a := 1; a <= y; a++ {
			for b := 1; b <= x; b++ {
				// prints top left corner
				if a == 1 && b == 1 {
					print("/")
					// prints top right corner
				} else if a == 1 && b == x {
					print("\\")
					// prints bottom left corner
				} else if a == y && b == 1 {
					print("\\")
					// prints bottom right corner
				} else if a == y && b == x {
					print("/")
					// prints left column
				} else if b == 1 && (a != y || a != 1) {
					print("*")
					// prints right column
				} else if b == x && (a != 1 || a != y) {
					print("*")
					// prints top row
				} else if a == 1 && (b != 1 || b != x) {
					print("*")
					//prints bottom row
				} else if a == y && (b != 1 || b != x) {
					print("*")
				} else {
					print(" ")
				}
			}
			print("\n")
		}
	}
}