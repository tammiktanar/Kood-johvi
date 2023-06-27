package dlx

type Root struct {
	allHeaders []*header
	*header
	wip       []*node
	solutions [][][]string
}

type header struct {
	name string

	nodeCount int
	*node
}

type node struct {
	me             *node
	header         *header
	up, dn, lt, rt *node
	slack          bool
	first          bool
}

func newNode() *node {
	new := node{}
	new.me = &new
	new.init()
	return new.me
}

func newHeader(name string) *header {
	new := header{node: newNode()}
	new.header = &new
	new.name = name
	return new.header
}

func (n *node) init() {
	n.up = n.me
	n.dn = n.me
	n.lt = n.me
	n.rt = n.me
}

func (n *node) insertNewNodeR(new *node) {
	n.rt.lt = new
	new.rt = n.rt

	n.rt = new
	new.lt = n.me
}

func (n *node) insertNewNodeD(new *node) {
	n.dn.up = new
	new.dn = n.dn

	n.dn = new
	new.up = n.me
}

func (n *node) removeLR() {
	n.lt.rt = n.rt
	n.rt.lt = n.lt
}

func (n *node) unRemoveLR() {
	n.lt.rt, n.rt.lt = n.me, n.me
}

func (n *node) removeUD() {
	n.up.dn = n.dn
	n.dn.up = n.up
}

func (n *node) unRemoveUD() {
	n.up.dn, n.dn.up = n.me, n.me
}
