import './style.scss';
import './local-style.scss';

/// import wasm module
import('./pkg').catch(console.error).then((module) => module.ray_tracer());

