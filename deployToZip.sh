#! bin/bash
# raspberry pi zero w
export PATH="/home/spoomer/dev/rust_linker/gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf/bin:$PATH"
cargo build -r --target arm-unknown-linux-gnueabihf
cd ./frontend_vue
npm run build
cd ..
mkdir release
cp -r ./wwwroot/ ./release/wwwroot/
cp ./target/arm-unknown-linux-gnueabihf/release/shelly_web ./release/shelly_web
cp ./config.json ./release/config.json.template
zip -rm release.zip release
