import './style.css'
import { JsRenderer, JsRendererBuilder } from 'wrend';

const main = () => {
  console.log({ JsRenderer })
  let renderer = JsRenderer.builder().add_fragment_shader_src("test", "main() {}").build();
  console.log({ renderer })
};

main();