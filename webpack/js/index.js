import init,{run} from "../../wasm/pkg/wgpu_wasm_test.js";
Error.stackTraceLimit = 100;
window.addEventListener("load", () => {
  init();
});
