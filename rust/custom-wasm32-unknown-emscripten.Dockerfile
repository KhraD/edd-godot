FROM ghcr.io/cross-rs/wasm32-unknown-emscripten:edge

COPY extension_api.json /opt/

ENV GDRUST_GODOT_API_JSON="/opt/extension_api.json"