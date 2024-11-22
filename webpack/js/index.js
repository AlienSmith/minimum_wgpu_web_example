import init,{run} from "../../wasm/pkg/wgpu_wasm_test.js";
Error.stackTraceLimit = 100;

//this is a top level await which will pause execution of the js until the await function is done
await init().then(() => {
    console.log("WASM Loaded");
});

