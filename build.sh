#!/bin/sh

rm -rf pkg

for arg in "$@"; do
  case "$arg" in
    --release)
      release=true
      ;;
  esac
done

if [ "$release" = true ]; then
  wasm-pack build --target=no-modules --release || exit 1
else
  wasm-pack build --target=no-modules --dev || exit 1
fi

cp config.json pkg/config.json
echo "Copied config.json to /pkg"

echo "Copying manifest v3 to pkg/"
cp manifest_v3.json pkg/manifest.json


printf "
const runtime = chrome.runtime || browser.runtime;

async function run() {
  await wasm_bindgen(runtime.getURL('RustyRythm_bg.wasm'));
}

run();
" >> pkg/run_wasm.js
