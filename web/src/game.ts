import * as wasm from "./wasm/app_bg.wasm";
import { __wbg_set_wasm } from "./wasm/app_bg.js";
__wbg_set_wasm(wasm);
export { main } from "./wasm/app_bg.js";
