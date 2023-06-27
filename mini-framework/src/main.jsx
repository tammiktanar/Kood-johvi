import './style.css'
import Reacty from "../framework/reacty.js" // Needed for jsx
import ReactyDom from "../framework/reacty-dom.js";
import Todo from "./todo.jsx";

ReactyDom.render(<Todo />, document.querySelector("#app"));
