// file: src/reacty.js
import { h } from 'snabbdom';

const createElement = (type, props = {}, ...children) => {
  // flatten the children
  // this to make todos.map(todo => <p>{todo}</p>) work in jsx
  // [['idly'], ['dosa', 'vada']] -> ['idly', 'dosa', 'vada']
  children = children.flat();

  // if type is a Class then
  // 1. create a instance of the Class
  // 2. call the render method on the Class instance
  if (type.prototype && type.prototype.isReactyClassComponent) {
    const componentInstance = new type(props);

    // remember the current vNode instance
    componentInstance.__vNode = componentInstance.render();

    // add hook to snabbdom virtual node to know whether it was added to the actual DOM
    componentInstance.__vNode.data.hook = {
      create: () => {
        componentInstance.componentDidMount()
      }
    }

    return componentInstance.__vNode;
  }
  // if type is a function then call it and return its value
  if (typeof (type) == 'function') {
    return type(props);
  }

  props = props || {};
  let dataProps = {};
  let eventProps = {};

  let dataset = {};
  let attrs = {};
  let styles = {};
  let classes = {};

  // This is to separate out the text attributes and event listener attributes
  for(let propKey in props) {
    // event props always start with on e.g. onClick, onChange etc.
    if (propKey.startsWith('on')) {
      // onClick -> click
      const event = propKey.substring(2).toLowerCase();

      eventProps[event] = props[propKey];
    }
    else if (propKey === "dataset") {
      dataset = props[propKey]
    }
    else if (propKey === "attrs") {
      attrs = props[propKey]
    }
    else if (propKey === "class") {
      classes = props[propKey]
    }
    else {
      dataProps[propKey] = props[propKey];
    }
  }

  // props -> snabbdom's internal text attributes
  // on -> snabbdom's internal event listeners attributes
  // class -> snabbdom's internal class attributes
  // attrs -> snabbdom's internal custom attributes
  // dataset -> snabbdom's internal data-* attributes
  return h(type, { props: dataProps, dataset: dataset, attrs: attrs, style: styles, 'class': classes,  on: eventProps }, children);
};

// component framework class
class Component {
  constructor() { }

  componentDidMount() { }

  setState(partialState) {
    // update the state by adding the partial state
    this.state = {
      ...this.state,
      ...partialState
    }
    // call the __updater function that ReactyDom gave
    Reacty.__updater(this);
  }

  render() { }
}

// add a static property to differentiate between a class and a function
Component.prototype.isReactyClassComponent = true;

// to be exported like React.createElement, React.Component
const Reacty = {
  createElement,
  Component
};

export default Reacty;
