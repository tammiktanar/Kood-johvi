/*---------------------------/
/  Olarols implementation of /
/  Knuth's Algorithm X 		 /
/  Using dancing links 		 /
/---------------------------*/

package dlx

import (
	"fmt"
)

func NewDance() *Root {
	root := Root{header: newHeader("root"), wip: make([]*node, 100)}

	return &root
}

func (r *Root) AddHeader(name string) {
	new := newHeader(name)
	r.lt.insertNewNodeR(new.me)
	r.allHeaders = append(r.allHeaders, new)
}

func (r *Root) AddGreedySubHeader(name string) {
	new := newHeader(name)
	r.lt.insertNewNodeR(new.me)
	r.allHeaders = append(r.allHeaders, new)

	slack := newNode()
	slack.slack = true
	slack.header = new
	new.insertNewNodeD(slack)
}

func (r *Root) AddSubHeader(name string) {
	new := newHeader(name)
	r.allHeaders = append(r.allHeaders, new)
}

func (r *Root) Solve() [][][]string {
	r.search(0)

	return r.solutions
}

func (r *Root) AddMatrix(data [][]byte) error {
	for _, row := range data {
		err := r.AddRow(row)
		if err != nil {
			return err
		}
	}
	return nil
}

func (r *Root) AddRow(row []byte) error {
	if len(r.allHeaders) != len(row) {
		return fmt.Errorf("unexpected data width - expected: %v; got: %v", len(r.allHeaders), len(row))
	}

	var first *node

	for j, b := range row {
		if b != 0 {
			new := newNode()
			new.header = r.allHeaders[j]

			new.header.up.insertNewNodeD(new.me)
			new.header.nodeCount++

			if first != nil {
				first.lt.insertNewNodeR(new.me)
			} else {
				new.first = true
				first = new.me
			}
		}
	}
	return nil
}

func (r *Root) search(level int) {
	if r.isComplete() {
		var sol [][]string
		for i := 0; i < level; i++ {
			res := r.wip[i].getRowNames()
			if len(res) > 0 {
				sol = append(sol, res)
			}
		}
		if len(sol) > 0 {
			r.solutions = append(r.solutions, sol)
		}
	} else {

		col := r.getSmallestColumn()
		col.cover()

		for row := col.dn; row != col.me; row = row.dn {

			for j := row.rt; j != row; j = j.rt {
				j.header.cover()
			}

			r.wip[level] = row
			r.search(level + 1)
			// r.wip = r.wip[:len(r.wip)-1]

			for j := row.lt; j != row; j = j.lt {
				j.header.unCover()
			}
		}
		col.unCover()
	}
}

func (c *header) cover() {
	c.removeLR()

	for i := c.dn; i != c.me; i = i.dn {
		for j := i.rt; j != i; j = j.rt {
			j.removeUD()
			j.header.nodeCount--
		}
	}
}

func (c *header) unCover() {
	for i := c.up; i != c.me; i = i.up {
		for j := i.lt; j != i; j = j.lt {
			j.header.nodeCount++
			j.unRemoveUD()
		}
	}

	c.unRemoveLR()
}

func (r *Root) isComplete() bool {
	return r.rt == r.me
}

func (r *Root) getSmallestColumn() *header {
	min := r.rt.header
	for i := min.rt.header; i != r.header; i = i.rt.header {
		if i.nodeCount < min.nodeCount {
			min = i
		}
	}
	return min
}

func (n *node) getRowNames() []string {
	if n.slack {
		return []string{}
	}

	a := n.getFirst()
	str := []string{a.header.name}
	for i := a.rt; i != a; i = i.rt {
		str = append(str, i.header.name)
	}
	return str
}

func (n *node) getFirst() *node {
	a := n
	for ; a != n.rt && !a.first; a = a.lt {
	}
	return a
}
