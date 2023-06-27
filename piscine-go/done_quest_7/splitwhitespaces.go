package SplitWhiteSpaces

func SplitWhiteSpaces(s string) []string {
	res := make([]string, 0)
	resTemp := ""
	for _, char := range s {
		if char == ' ' && resTemp != "" {
			res = append(res, resTemp)
			resTemp = ""
		} else {
			if char != ' ' {
				resTemp += string(char)
			}
		}
	}
	if resTemp != "" {
		res = append(res, resTemp)
		resTemp = ""
	}

	return res
}
