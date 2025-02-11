
const runtime = chrome.runtime || browser.runtime;

async function run() {
  await wasm_bindgen(runtime.getURL('amazing_extension_bg.wasm'));
}

run();
