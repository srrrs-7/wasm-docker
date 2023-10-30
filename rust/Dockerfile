FROM scratch
COPY ./app/target/wasm32-wasi/release/app.wasm ./app.wasm

ENTRYPOINT [ "./app.wasm" ]