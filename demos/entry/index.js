import './style.scss';

/// import wasm module (main functino auto starts)
import('./pkg').catch(console.error).then((module) => module.kernels_main());

