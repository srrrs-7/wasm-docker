FROM scratch
COPY ./app/target/wasm32-wasi/debug/app.wasm ./app.wasm

ENTRYPOINT [ "./app.wasm" ]