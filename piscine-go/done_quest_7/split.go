package Split

func Split(s, sep string) []string {
	res := make([]string, 0)
	resTemp := ""
	resTemp2 := ""
	check := []rune(sep)
	index := 0
	if len(check) >= 2 {
		for _, char := range s {
			if char == check[index] {
				if index == len(check)-1 {
					res = append(res, resTemp)
					resTemp2 = ""
					resTemp = ""
					index = -1
				} else {
					resTemp2 += string(char)
				}
				index++
			} else {
				index = 0
				resTemp += resTemp2 + string(char)
				resTemp2 = ""
			}
		}
		if resTemp != "" {
			if resTemp2 != "" {
				resTemp += resTemp2
			}
			res = append(res, resTemp)
		}
	} else {
		for _, char := range s {
			if string(char) == s && resTemp != "" {
				res = append(res, resTemp)
				resTemp = ""
			} else {
				if string(char) != s {
					resTemp += string(char)
				}
			}
		}
		if resTemp != "" {
			res = append(res, resTemp)
			resTemp = ""
		}
	}

	return res
}
