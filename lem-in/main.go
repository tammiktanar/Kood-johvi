package main

import (
	"fmt"
	"lem-in/dlx"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type Room struct {
	name        string
	x, y        int
	connections []*Room
	antCount    int
}

func main() {
	if len(os.Args) != 2 {
		log.Fatalln("invalid number of arguments, need 1 input file path")
	}
	start, end, input := readInputFile(os.Args[1])
	if start.antCount <= 0 {
		log.Fatalf("cannot send %v ants", start.antCount)
	}
	allPaths := FindPaths(start, end)
	if len(allPaths) == 0 {
		log.Fatalln("could not find a path from start to end")
	}

	fmt.Println(input)

	if len(allPaths[0]) == 2 {
		specialOutput(start, end)
		return
	}

	solutions := FindCombos(allPaths)
	output(solutions, start, end)
}

func readInputFile(filename string) (*Room, *Room, string) {
	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatalln(err)
	}

	lines := strings.Split(strings.ReplaceAll(string(data), "\r\n", "\n"), "\n")

	var start string
	var end string
	var antCount int
	rooms := make(map[string]*Room)
	nextStart := false
	nextEnd := false

	for i, line := range lines {
		switch {
		case i == 0:
			antCount = atoiNoErr(line)
		case line == "##start":
			nextStart = true
		case line == "##end":
			nextEnd = true
		case strings.HasPrefix(line, "#") || line == "":
		case strings.Contains(line, " "):
			roomSlice := strings.Split(line, " ")
			name := roomSlice[0]

			rooms[name] = &Room{
				name: name,
				x:    atoiNoErr(roomSlice[1]),
				y:    atoiNoErr(roomSlice[2]),
			}

			if nextStart {
				start = name
				rooms[name].antCount = antCount
				nextStart = false
			}
			if nextEnd {
				end = name
				nextEnd = false
			}
		case strings.Contains(line, "-"):
			names := strings.Split(line, "-")

			a, exist := rooms[names[0]]
			if !exist {
				log.Fatalf(`can't make a connection to room "%v"`, names[0])
			}

			b, exist := rooms[names[1]]
			if !exist {
				log.Fatalf(`can't make a connection to room "%v"`, names[1])
			}

			a.connections = append(a.connections, b)
			b.connections = append(b.connections, a)
		}
	}

	if start == "" {
		log.Fatalln("could not find start room")
	}
	if start == "" {
		log.Fatalln("could not find end room")
	}

	return rooms[start], rooms[end], string(data)
}

func atoiNoErr(s string) int {
	out, err := strconv.Atoi(s)
	if err != nil {
		log.Fatalln(err)
	}
	return out
}

func FindPaths(start, end *Room) (allPaths [][]*Room) {
	findPath(end, []*Room{start}, &allPaths)
	return
}

func findPath(end *Room, visited []*Room, allPaths *[][]*Room) {
	if end.isInSet(visited[len(visited)-1].connections) {
		visited = append(visited, end)
		*allPaths = append(*allPaths, visited)
	} else {
		for _, nextRoom := range visited[len(visited)-1].connections {
			if !isValid(nextRoom, visited, end) {
				continue
			}

			newVisited := make([]*Room, len(visited))
			copy(newVisited, visited)
			newVisited = append(newVisited, nextRoom)
			findPath(end, newVisited, allPaths)
		}
	}
}

func isValid(next *Room, visited []*Room, end *Room) bool {
	if next == visited[0] {
		return false
	}

	for _, nextNeigbour := range next.connections {
		if nextNeigbour.isInSet(visited[:len(visited)-1]) {
			return false
		}
	}

	return true
}

func (room *Room) isInSet(set []*Room) bool {
	for _, check := range set {
		if check == room {
			return true
		}
	}
	return false
}

func FindCombos(allPaths [][]*Room) [][][]string {
	dance := dlx.NewDance()                  // Create new dancing links object...
	uniques := buildHeaders(dance, allPaths) // ...make headers for it...
	buildRows(allPaths, uniques, dance)      // ...and give it rows to work on

	solutions := dance.Solve()
	return solutions
}

func buildHeaders(dance *dlx.Root, allPaths [][]*Room) map[string]int {
	unique := make(map[string]int)
	for _, path := range allPaths {
		for _, room := range path[1 : len(path)-1] {
			_, exist := unique[room.name]

			if !exist {
				i := len(unique)
				unique[room.name] = i
				dance.AddGreedySubHeader(room.name)
			}
		}
	}

	return unique
}

func buildRows(allPaths [][]*Room, unique map[string]int, dance *dlx.Root) {
	row := make([]byte, len(unique))
	for _, path := range allPaths {
		for _, room := range path[1 : len(path)-1] {
			row[unique[room.name]] = 1
		}
		err := dance.AddRow(row)
		if err != nil {
			log.Fatalln(err)
		}

		for i := range row {
			row[i] = 0
		}
	}
}

func antSim(combo [][]string, start, end *Room, bestTime int) {
	ants := start.antCount
	senders := []*sender{}

	sendCapacity := 0
	for _, path := range combo {
		sendCapacity += (bestTime + 1 - len(path))
	}
	overflow := len(combo) - (sendCapacity % ants)

	namer := newCounter()
	var antList []*ant
	for _, path := range combo {
		sender := &sender{path: path, start: start, nameGen: namer, antList: &antList}

		sender.antsToSend = bestTime - len(path)
		if overflow > 0 {
			sender.antsToSend++
			overflow--
		}

		senders = append(senders, sender)
	}

	for end.antCount < ants {
		for i := 0; i < len(senders); i++ {
			finished := senders[i].sendAnt()
			if finished {
				senders = append(senders[:i], senders[i+1:]...)
				i--
			}
		}

		var notFirst bool
		for j := 0; j < len(antList); j++ {
			finished := antList[j].move()

			if notFirst {
				fmt.Print(" ")
			} else {
				notFirst = true
			}
			fmt.Printf("L%v-%v", antList[j].name, antList[j].current.name)

			if finished {
				antList = append(antList[:j], antList[j+1:]...)
				j--
			}
		}
		fmt.Println()
	}
}

type sender struct {
	antsToSend int
	nameGen    func() int
	path       []string
	antList    *[]*ant
	start      *Room
}

type ant struct {
	name    int
	current *Room
	path    []string
}

func (s *sender) sendAnt() bool {
	if s.antsToSend > 0 {
		pathCopy := make([]string, len(s.path))
		copy(pathCopy, s.path)
		newAnt := &ant{
			name:    s.nameGen(),
			path:    pathCopy,
			current: s.start,
		}
		*s.antList = append(*s.antList, newAnt)
		s.antsToSend--
	}

	if s.antsToSend > 0 {
		return false
	}
	return true
}

func (a *ant) move() bool {
	oldRoom := a.current
outer:
	for i, nextRoomName := range a.path {
		for _, connectedRoom := range a.current.connections {
			if nextRoomName == connectedRoom.name {
				a.path = append(a.path[:i], a.path[i+1:]...)
				a.current = connectedRoom
				break outer
			}
		}
	}

	if a.current == oldRoom {
		panic(fmt.Errorf("the ant #%v is unable to move at %v while looking for %v", a.name, a.current.name, a.path))
	}

	if len(a.path) == 0 {
		a.current.antCount++
		return true
	}
	return false
}

func newCounter() func() int {
	var count int
	return func() int {
		count++
		return count
	}
}

func specialOutput(start, end *Room) {
	var notFirst bool
	for i := 1; i <= start.antCount; i++ {
		if notFirst {
			fmt.Print(" ")
		} else {
			notFirst = true
		}
		fmt.Printf("L%v-%v", i, end.name)
	}
	fmt.Println()
}

func output(allCombos [][][]string, start, end *Room) {
	ants := start.antCount
	var bestCombo [][]string
	bestTime := 0.0

	for _, combo := range allCombos {
		totalLength := 0
		for _, path := range combo {
			totalLength += len(path) + 1
		}
		time := float64(totalLength+ants)/float64(len(combo)) - 1

		if time <= bestTime || bestTime == 0 {
			bestTime = time
			bestCombo = combo
		}

	}
	bestTime = math.Ceil(bestTime)

	for i := range bestCombo {
		bestCombo[i] = append(bestCombo[i], end.name)
	}

	antSim(bestCombo, start, end, int(bestTime))
}
