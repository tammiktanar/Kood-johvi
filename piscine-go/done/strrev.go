package StrRev

func StrRev(s string) (result string) {
	for _, v := range s {
		result = string(v) + result
	}
	return
}
