#! bin/bash
# raspberry pi zero w
export PATH="/home/spoomer/dev/rust_linker/gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf/bin:$PATH"
cargo build -r --target arm-unknown-linux-gnueabihf
podman run --rm -v "$PWD":/usr/src/app -w /usr/src/app/frontend_vue node npm install
podman run --rm -v "$PWD":/usr/src/app -w /usr/src/app/frontend_vue node npm run build
mkdir release
cp -r ./wwwroot/ ./release/wwwroot/
cp ./target/arm-unknown-linux-gnueabihf/release/shelly_web ./release/shelly_web
cp ./config.json ./release/config.json.template
zip -rm release.zip release
