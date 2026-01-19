import { signal, createRustSignal } from '../dist/index.js';

// JS implementation
const jsSignal = signal(0);
console.log('JS Signal value:', jsSignal.value);
jsSignal.value = 5;
console.log('JS Signal updated value:', jsSignal.value);

// Rust/WASM implementation
const rustSignal = await createRustSignal(0);
console.log('Rust Signal value:', rustSignal.value);
rustSignal.value = 10;
console.log('Rust Signal updated value:', rustSignal.value);

// Compare performance
console.time('JS Signal');
for (let i = 0; i < 10000; i++) {
  jsSignal.value = i;
}
console.timeEnd('JS Signal');

console.time('Rust Signal');
for (let i = 0; i < 10000; i++) {
  rustSignal.value = i;
}
console.timeEnd('Rust Signal');
