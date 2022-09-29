import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default {
  optimizeDeps: {
    // must exclude wrend from Vite's automatic optimization 
    // to prevent initialization weird errors from wasm
    exclude: ['wrend']
  },
  plugins: [
    wasm(),
    topLevelAwait()
  ]
};