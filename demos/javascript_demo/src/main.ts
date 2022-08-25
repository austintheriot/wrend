import './style.css'
import { Renderer } from 'wrend';

const main = () => {
  const canvas = document.querySelector('canvas') as HTMLCanvasElement;

  const renderer = Renderer.builder().set_canvas(canvas).set_render_callback(() => {
    console.log('Render callback being called with the renderer itself', { renderer });
  }).build();

  console.log({ renderer })
  renderer.render();
};

main();