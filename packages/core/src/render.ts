import { effect } from "./reactivity"


type VNode = {
  type: string | Function
  props: Record<string, any>
  children: any[]
}

export function h(type: string | Function, props: any, ...children: any[]): VNode {
  return {
    type,
    props: props || {},
    children: children.flat()
  }
}

export function render(vnode: VNode, container: HTMLElement) {
  // 清空容器
  container.innerHTML = ''
  
  // 创建元素
  const element = createElement(vnode)
  if (element) {
    container.appendChild(element)
  }
}

function createElement(vnode: VNode): Node | null {
  if (typeof vnode.type === 'function') {
    // 如果是组件函数，调用它
    return createElement(vnode.type(vnode.props))
  }
  
  if (typeof vnode.type === 'string') {
    const element = document.createElement(vnode.type)
    
    // 设置属性
    for (const [key, value] of Object.entries(vnode.props)) {
      if (key.startsWith('on') && typeof value === 'function') {
        element.addEventListener(key.slice(2).toLowerCase(), value)
      } else if (key !== 'children') {
        element.setAttribute(key, String(value))
      }
    }
    
    // 添加子节点
    for (const child of vnode.children) {
      if (typeof child === 'string' || typeof child === 'number') {
        element.appendChild(document.createTextNode(String(child)))
      } else if (child && typeof child === 'object') {
        const childElement = createElement(child)
        if (childElement) {
          element.appendChild(childElement)
        }
      }
    }
    
    return element
  }
  
  return null
}

// 响应式渲染辅助函数
export function reactiveRender(renderFn: () => VNode, container: HTMLElement) {
  effect(() => {
    const vnode = renderFn()
    render(vnode, container)
  })
}
