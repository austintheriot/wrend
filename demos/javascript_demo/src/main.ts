import './style.css'
import { JsRenderer } from 'wrend';

const main = () => {
  const canvas = document.querySelector('canvas') as HTMLCanvasElement;

  const renderer = JsRenderer.builder().set_canvas(canvas).set_render_callback(() => {
    console.log('Render callback being called with the renderer itself', { renderer });
  }).build();

  console.log({ renderer })

  renderer.render();
};

main();