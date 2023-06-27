package ConcatParams

func ConcatParams(args []string) string {
	res := ""
	for i := 0; i < len(args); i++ {
		if i == (len(args) - 1) {
			res += args[i]
		} else {
			res += args[i] + "\n"
		}
	}

	return res
}
