import './style.css'
import { JsRenderer, JsRendererBuilder } from 'wrend';

const main = () => {
  console.log({ JsRenderer })
  const renderer = JsRenderer.builder();
  console.log({ renderer })
};

main();