package Join

func Join(strs []string, sep string) string {
	res := strs[0]
	for i := 1; i < len(strs); i++ {
		res += sep + strs[i]
	}
	return res
}
