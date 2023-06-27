package piscine

import "github.com/01-edu/z01"

func EightQueens() {
	board := make([]int, 0, 8)
	placeQueen(board)
}

func placeQueen(old []int) {
	board := make([]int, len(old)+1, cap(old))
	copy(board, old)
	row := len(old)
	for col := 0; col < cap(old); col++ {
		fits := true
		for row_, col_ := range old {
			if col == col_ || // if in same column
				col+row == col_+row_ || // or same diagonal
				(cap(old)-1-col)+row == (cap(old)-1-col_)+row_ { // or same second diagonal
				fits = false
				break
			}
		}
		if fits {
			board[row] = col
			if len(board) >= cap(board) {
				printBoard(board)
			} else {
				placeQueen(board)
			}
		}
	}
}

func printBoard(board []int) {
	for _, v := range board {
		z01.PrintRune(rune('0' + v + 1))
	}
	z01.PrintRune(rune('\n'))
}
