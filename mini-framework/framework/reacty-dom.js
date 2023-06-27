// file: src/reacty-dom.js
import * as snabbdom from 'snabbdom';
import {propsModule, eventListenersModule, datasetModule, classModule, attributesModule} from 'snabbdom'
import Reacty from './reacty';

// propsModule -> this helps in patching text attributes
// eventListenersModule -> this helps in patching event attributes
const reconcile = snabbdom.init([propsModule, eventListenersModule, datasetModule, classModule, attributesModule]);
// we need to maintain the latest rootVNode returned by render
let rootVNode;

// Attaches a component to a given element in the DOM
const render = (el, rootDomElement) => {
  // logic to put el into the rootDomElement
  // ie. ReactyDom.render(<App />, document.getElementById('root'));
  // happens when we call render for the first time
  if (rootVNode == null) {
    rootVNode = rootDomElement;
  }

  // remember the VNode that reconcile returns
  rootVNode = reconcile(rootVNode, el);
}

// ReactyDom telling React how to update DOM
Reacty.__updater = (componentInstance) => {
  // logic on how to update the DOM when you call this.setState

  // get the oldVNode stored in __vNode
  const oldVNode = componentInstance.__vNode;
  // find the updated DOM node by calling the render method
  const newVNode = componentInstance.render();

  // update the __vNode property with updated __vNode
  componentInstance.__vNode = reconcile(oldVNode, newVNode);
}

// to be exported like ReactDom.render
const ReactyDom = {
  render
};

export default ReactyDom;
