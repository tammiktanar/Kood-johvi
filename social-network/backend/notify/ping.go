package notify

type Ping struct{}

func (p Ping) Targets() []int64 {
	slc := make([]int64, 0, 25)
	for i := int64(1); i <= 3; i++ {
		slc = append(slc, i)
	}
	return slc
}

func (p Ping) Message() string {
	return "ping"
}

func (p Ping) Links() []Link {
	return nil
}
