## Overview

Reacty is a small react-like framework. 
It doesn't aim to be the most user-friendly framework, just simple in its implementation.

### Features
- Virtual DOM - Virtual DOM is used to synchronise state with the actual DOM.
- Event handling - Events are defined in jsx using `onSomeEvent` attributes.

## Setup

1. Set up an empty project with a bundler of your choice.
2. Copy Reacty's `framework` directory into your project.
3. Set your bundler's JSX factory to `Reacty.createElement`

Example of setting the JSX factory in Vite:

```js
// vite.config.js
import {defineConfig} from 'vite'

export default defineConfig({
	esbuild: {
		jsxFactory: 'Reacty.createElement',
	}
})
```

## Usage
### Creating a component

Creating a component is as simple as creating a class that inherits from `Reacty.Component`. Note that you need to do
this inside a `.jsx` file.

The component has a couple of special methods that you should be aware of:
- `constructor()` - Initialize your component code in here. Don't forget to set state at the end.
- `componentDidMount()` - This function gets called when your component is mounted into the DOM.
- `render()` - Return the html rendering template from this function.
- `setState(newState)` - This function should be called by your code whenever you need to update state.

### Displaying the component
To display your component, you need to use the `ReactyDom.render(component, element)` function. 
It will append the html rendered by the component inside the given element.

Make sure you also import Reacty here, or it won't work correctly.

## Examples
### Initialize root component
```jsx
import Reacty from "../framework/reacty.js" // Needed for jsx
import ReactyDom from "../framework/reacty-dom.js";
import MyComponent from "./myComponent.jsx";

ReactyDom.render(<MyComponent />, document.querySelector("#app"));
```

### Creating component
```jsx
// myComponent.jsx
import Reacty from '../framework/reacty.js';

export default class MyComponent extends Reacty.Component {
	constructor(props) {
		super(props)

		// Setup code goes here
		let foo = "hello"
		this.bar = foo + " world"

		// Make sure to set the state at the end
		this.state = {message: this.bar}
	}

	// This method is called when the component is mounted
	componentDidMount() {
		console.log('Component mounted');
	}

	// You can also add any custom functions you need.
	myMethod(e) {
		this.bar += "!"

		// Use the setState() function to update component state
		this.setState({
			message: this.bar
		})
	}

	// The render method sets the template for rendering your HTML
	render() {
		return (
			<div>
				<span>{this.bar}</span><br/>
				<input type="button" value="click-me" onClick={e => this.myMethod(e)} />
			</div>
		)
	}
}
```