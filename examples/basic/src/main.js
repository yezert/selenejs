
import {
  signal,
  effect,
  computed,
  h,
  compileTemplate,
  Fragment,
  createApp,
  createRustSignal,
} from '@selene/core'

// 1. 创建响应式状态
const count = signal(0)
const name = signal('World')

// 2. 使用编译器，把模板编译成渲染函数
const templateVersion = signal(0) // 仅用于触发重新渲染
let Counter = () => h('div', { class: 'counter' }, 'Compiling template...')

async function setupCompiledCounter() {
  const tpl = `
  <div class="counter">
    <h1>Hello, \${name.value}!</h1>
    <p>Count: \${count.value}</p>
    <div class="actions">
      <button onClick="{() => count.value++}">Increment</button>
      <button onClick="{() => (count.value = 0)}">Reset</button>
    </div>
  </div>
  `

  let compiled = null
  try {
    compiled = await compileTemplate(tpl)
  } catch (e) {
    console.error('compileTemplate failed:', e)
    compiled = null
  }

  if (!compiled) {
    Counter = () =>
      h('div', { class: 'counter' }, 'Compile failed (see console).')
    templateVersion.value++
    return
  }

  // 让编译后的函数可以访问 h / Fragment / 信号
  // eslint-disable-next-line no-new-func
  Counter = new Function(
    'h',
    'Fragment',
    'count',
    'name',
    `return (${compiled});`
  )(h, Fragment, count, name)

  // 触发视图刷新（依赖 templateVersion 的地方会重新执行）
  templateVersion.value++
}

setupCompiledCounter()

// 3. 使用 createApp 简化挂载逻辑
createApp(() => {
  templateVersion.value // 依赖，模板完成编译后会刷新
  return Counter()
}).mount('#app')

// 调试日志
effect(() => {
  console.log('Count changed:', count.value)
})

// 计算属性示例
const doubled = computed(() => count.value * 2)
effect(() => {
  console.log('Doubled:', doubled.value)
})

// 4. RustSignal 示例：在控制台定期打印 Rust 侧计数
async function demoRustSignal() {
  const rustSignal = await createRustSignal(0)
  console.log('RustSignal initial =', rustSignal.value)
  setInterval(() => {
    rustSignal.value++
    console.log('RustSignal value =', rustSignal.value)
  }, 1000)
}

demoRustSignal()
