import './style.scss';

/// import wasm module
import('./pkg').catch(console.error).then((module) => module.multiple_neighborhood_cellular_automata());

