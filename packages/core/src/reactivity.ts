export type Effect = () => void

// 当前正在执行的 effect
let activeEffect: Effect | null = null

export class Signal<T> {
  private _value: T
  private _subscribers: Set<Effect> = new Set()

  constructor(initialValue: T) {
    this._value = initialValue
  }

  get value(): T {
    // 收集依赖
    if (activeEffect) {
      this._subscribers.add(activeEffect)
    }
    return this._value
  }

  set value(newValue: T) {
    if (Object.is(this._value, newValue)) return
    this._value = newValue
    
    // 通知所有订阅者
    for (const effect of this._subscribers) {
      effect()
    }
  }

  subscribe(effect: Effect): () => void {
    this._subscribers.add(effect)
    return () => this._subscribers.delete(effect)
  }
}

export function signal<T>(initialValue: T): Signal<T> {
  return new Signal(initialValue)
}

export function effect(fn: Effect): () => void {
  const effectFn = () => {
    activeEffect = effectFn
    fn()
    activeEffect = null
  }
  
  effectFn()
  
  // 返回清理函数
  return () => {
    // 从所有信号中移除这个 effect
    // （简化版，后面会优化）
  }
}

export function computed<T>(fn: () => T): Signal<T> {
  const signal = new Signal(fn())
  
  // 当依赖变化时重新计算
  effect(() => {
    signal.value = fn()
  })
  
  return signal
}
