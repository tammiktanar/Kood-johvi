import Reacty from '../framework/reacty.js';

export default class Todo extends Reacty.Component {
    constructor(props) {
        super(props);

        let archive = [], // Notice change here
            keys = Object.keys(localStorage),
            i = keys.length

        while ( i-- ) {
            let cur_item = localStorage.getItem( keys[i] )

            if (!isNaN(keys[i])) {
                archive.push(JSON.parse(cur_item))
            }
        }

        let cur_todos = archive || []


        this.state = {
            count: 0,
            todos: cur_todos,
            cur_page: window.location.hash
        }
    }

    componentDidMount() {
        console.log('Component mounted');
    }

    createNewTodo(input) {
        if (input.value !== undefined && input.value !== "") {
            let currentTodos = this.state.todos
            currentTodos.push({id: Date.now(), name: input.value, status: false})

            this.setState({
                todos: currentTodos
            })
            localStorage.setItem(currentTodos.at(-1).id, JSON.stringify(currentTodos.at(-1)))
            input.value = ""
        }
    }

    removeTodo(id) {
        let currentTodos = this.state.todos
        currentTodos.splice(currentTodos.findIndex(todo => todo.id === id), 1)

        this.setState({
            todos: currentTodos
        })

        localStorage.removeItem(id)
    }

    checkTodo(id) {
        let currentTodos = this.state.todos
        let index = currentTodos.findIndex(todo => todo.id === id)

        currentTodos[index].status = !currentTodos[index].status
        localStorage.setItem(currentTodos[index].id, JSON.stringify(currentTodos[index]))

        this.setState({
            todos: currentTodos
        })


        if (this.state.cur_page == "#/active") {
            this.clearCheckboxes()
        } else if (this.state.cur_page == "#/completed") {
            this.checkCheckboxes()
        }
    }

    toggleAll(){
        this.clearCheckboxes()

        let editedTodos = this.state.todos

        let setVal = false
        if (editedTodos.find(todo => !todo.status)) {
            setVal = true
            this.checkCheckboxes()

        }


        editedTodos.forEach((todo) => {
            todo.status = setVal
            localStorage.setItem(todo.id, JSON.stringify(todo))
        })


        this.setState({
            todos: editedTodos
        })


    }

    clearCompleted(){

        let clearedTodos = this.state.todos
        .filter(todo => {
            if (todo.status !== true) {
                return true
            } else {
                localStorage.removeItem(todo.id)
                return false
            }})

        this.clearCheckboxes()

        this.setState({
            todos: clearedTodos
        })
    }

    clearCheckboxes() {
        let allCheckboxes = document.querySelectorAll(".toggle")
        if (allCheckboxes) {
            allCheckboxes.forEach(element => {
                element.checked = false
            });
        }
    }

    checkCheckboxes() {
        let allCheckboxes = document.querySelectorAll(".toggle")
        if (allCheckboxes) {
            allCheckboxes.forEach(element => {
                element.checked = true
            });
        }
    }

    checkInput(e, name) {
        if(e.key === 'Enter') {
            if (name == "new-todo") {
                this.createNewTodo(e.target)
            } else if (name == "edit-todo") {
                this.editTodo(e.target)
            }
        }
    }

    updateCurrentState(e) {
        let target_hash = e.target.getAttribute("href")
        let individual = false
        let set_checked = false

        switch (target_hash) {
            case "#/completed":
                set_checked = true
                break;
            case "#/active":
                set_checked = false
                break;
            default:
                individual = true
                break;
        }
        

        let allCheckboxes = document.querySelectorAll(".toggle")
        if (allCheckboxes) {
            allCheckboxes.forEach((element, i) => {
                if (individual) {
                    element.checked = this.state.todos[i].status
                } else {
                    element.checked = set_checked
                }
            });
        }

        this.setState({
            cur_page: target_hash
        })
    }

    setEditingInput(id) {
        let currentTodos = this.state.todos
        let index = currentTodos.findIndex(todo => todo.id === id)

        currentTodos[index].editing = true

        this.setState({
            todos: currentTodos
        })

        let intervalId = setInterval(() => {
            let input = document.querySelector("#editingInput");

            if (input) {
                input.focus()
                clearInterval(intervalId)
            }
        }, 1);
    }

    setUnEditingInput (event, id) {
        event.target.blur()
        let currentTodos = this.state.todos
        let index = currentTodos.findIndex(todo => todo.id === id)

        currentTodos[index].editing = undefined

        this.setState({
            todos: currentTodos
        })
    }

    editTodo (element) {
        let currentTodos = this.state.todos
        let index = currentTodos.findIndex(todo => todo.editing === true)

        if (index != undefined) {
            currentTodos[index].name = element.value
            currentTodos[index].editing = undefined

            localStorage.setItem(currentTodos[index].id, JSON.stringify(currentTodos[index]))


            this.setState({
                todos: currentTodos
            })

        }
    }

    render() {
        return (

            <section className="todoapp">


                <header className="header">
                    <h1>todos</h1>
                    <input className="new-todo" placeholder="What needs to be done?" autoFocus onKeyDown={(e) => this.checkInput(e, "new-todo")}/>
                </header>

                <section className="main" style={this.state.todos.length > 0 ? "display: block;" : "display: none;"}>
                    <input id="toggle-all" className="toggle-all" type="checkbox" defaultChecked={!this.state.todos.find(todo => !todo.status )}/>
                    <label htmlFor="toggle-all" onClick={() => this.toggleAll()}>Mark all as complete</label>
                    <ul className="todo-list">
                        {this.state.todos.filter(todo => {
                            switch (this.state.cur_page) {
                                case "#/completed":
                                    return !!todo.status;
                                case "#/active":
                                    return !todo.status;
                                default:
                                    return true
                            }
                        }).map(todo =>
                            <li
                                {...{ "customattribute": "somevalue" }}
                                dataset={{ id: todo.id }}
                                className={todo.status ? todo.editing ?  'completed editing' : 'completed' : todo.editing ? 'editing' : ""}
                            >


                                <div className="view">
                                    <input className="toggle" onClick={() => this.checkTodo(todo.id)} type="checkbox" defaultChecked={todo.status}></input>

                                    <label ondblclick={() => {this.setEditingInput(todo.id)}}>{todo.name}</label>

                                    <button className="destroy" onClick={() => this.removeTodo(todo.id)}></button>
                                </div>

                                {todo.editing ?
                                    <input id="editingInput"
                                        onfocusout={(event) => this.setUnEditingInput(event, todo.id)}
                                        onKeyDown = {(e) => this.checkInput(e, "edit-todo")}
                                        className="edit"
                                        value={todo.name}/>
                                : ""}
                            </li>
                        )}


                        <footer className="footer" style="display: block;">
                            <span className="todo-count"><strong>{this.state.todos.filter(todo => !todo.status).length}</strong> items left</span>


                            <ul className="filters">
                                <li>
                                    <a href="#/" onClick={(e) => {this.updateCurrentState(e)}} className={this.state.cur_page !== "#/completed" ? this.state.cur_page !== "#/active" ? "selected" : "" : ""} >All</a>
                                </li>
                                <li>
                                    <a href="#/active" onClick={(e) => {this.updateCurrentState(e)}} className={this.state.cur_page === "#/active" ? "selected" : ""} >Active</a>
                                </li>
                                <li>
                                    <a href="#/completed" onClick={(e) => {this.updateCurrentState(e)}} className={this.state.cur_page === "#/completed" ? "selected" : ""} >Completed</a>
                                </li>
                            </ul>

                            <button className="clear-completed" onClick={() => this.clearCompleted()} style={this.state.todos.find(todo => todo.status ) ? "display: block;" :"display: none;"}>Clear completed</button>
                        </footer>

                    </ul>
                </section>
            </section>
        )
      }

}
