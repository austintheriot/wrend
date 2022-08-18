import './style.scss';

/// import wasm module
import('./pkg').catch(console.error).then((module) => module.recording_demo());

