package Index

func Index(s string, toFind string) int {
	res := -1
	found := false
	if toFind != "" {
		if s == toFind {
			res = 0
		} else {
			runeS := []rune(s)
			runeToFind := []rune(toFind)
			for i := 0; i < len(runeS); i++ {
				if !found {
					if runeS[i] == runeToFind[0] {
						for k := 0; k < len(runeToFind); k++ {
							if runeS[i+k] == runeToFind[k] {
								found = true
							} else {
								found = false
								break
							}
						}
					} else {
						res = -1
					}
				} else {
					res = i - 1
					break
				}
			}
		}
	} else {
		res = 0
	}

	return res
}
