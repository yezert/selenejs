declare module "./rust/selene_core.js" {
  export default function init(): Promise<void>;
  export class JsSignal {
    constructor(initialValue: number);
    get value(): number;
    set value(value: number);
  }
  export function compile_template(input: string): string;
}

// Allow importing without the .js extension in TS/Node resolution
declare module "./rust/selene_core" {
  export default function init(): Promise<void>;
  export class JsSignal {
    constructor(initialValue: number);
    get value(): number;
    set value(value: number);
  }
  export function compile_template(input: string): string;
}

declare module "@selene/core/rust" {
  export class JsSignal {
    constructor(initialValue: number);
    get value(): number;
    set value(value: number);
  }

  export class VNode {
    constructor();
    element(nodeType: string): VNode;
    text(content: string): VNode;
    set_prop(key: string, value: string): VNode;
    add_child(child: VNode): VNode;
    set_text(content: string): VNode;
  }

  export class Renderer {
    constructor();
    render(vnode: VNode, container: HTMLElement): void;
  }

  export function h(nodeType: string): VNode;
  export function reactive_render(renderFn: () => VNode, container: HTMLElement): void;
  export function compile_template(input: string): string;
}
