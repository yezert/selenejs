
import {
  signal,
  effect,
  h,
  reactiveRender,
  computed,
  compileTemplate,
  Fragment,
} from '@selene/core'

// 创建响应式状态
const count = signal(0)
const name = signal('World')
// 触发重新渲染用：当 Counter 从占位函数切换为编译产物时 bump 一下
const templateVersion = signal(0)

// 使用编译器：将模板编译为渲染函数（返回 VNode）
let Counter = () => h('div', { class: 'counter' }, 'Compiling template...')

// 编译阶段（异步）：只做一次
;(async () => {
  const tpl = `
  <div class="counter">
    <h1>Hello, \${name.value}!</h1>
    <p>Count: \${count.value}</p>
    <div class="actions">
      <button data-action="inc">Increment</button>
      <button data-action="reset">Reset</button>
    </div>
  </div>
  `

  // 编译器输出：`() => h(...)` 的函数源代码字符串
  let compiled = null
  try {
    compiled = await compileTemplate(tpl)
  } catch (e) {
    compiled = null
    console.error('compileTemplate failed:', e)
  }
  if (!compiled) {
    Counter = () =>
      h('div', { class: 'counter' }, 'Compile failed (see console).')
    templateVersion.value++
    return
  }

  // 让生成的函数能访问到 h/Fragment（多根节点时会用到 Fragment）
  // eslint-disable-next-line no-new-func
  Counter = new Function(
    'h',
    'Fragment',
    'count',
    'name',
    `return (${compiled});`
  )(h, Fragment, count, name)
  // 触发 reactiveRender 重新执行一次（否则会一直停留在占位 UI）
  templateVersion.value++

  // 绑定一次事件（事件委托，避免每次渲染重复绑定）
  const app = document.getElementById('app')
  if (app && !app._selene_events_attached) {
    app.addEventListener('click', (e) => {
      const t = e.target
      if (!(t instanceof Element)) return
      if (t.matches('[data-action="inc"]')) count.value++
      if (t.matches('[data-action="reset"]')) count.value = 0
    })
    app._selene_events_attached = true
  }
})()

// 响应式渲染：依赖变化时自动重新渲染
reactiveRender(() => {
  // 让 reactiveRender 依赖这个 signal，从而在模板编译完成时刷新 UI
  templateVersion.value
  return Counter()
}, document.getElementById('app'))

// 调试日志
effect(() => {
  console.log('Count changed:', count.value)
})

// 测试：每秒自动更新一次
setTimeout(() => {
  name.value = 'Selene'
}, 1000)

// 计算属性示例
const doubled = computed(() => count.value * 2)
effect(() => {
  console.log('Doubled:', doubled.value)
})
