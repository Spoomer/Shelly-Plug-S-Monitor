#! bin/bash
# raspberry pi zero w
cargo build -r --target arm-unknown-linux-gnueabihf
cd ./frontend_vue
npm run build
cd ..
mkdir release
cp -r ./wwwroot/ ./release/wwwroot/
cp ./target/arm-unknown-linux-gnueabihf/release/shelly_web ./release/shelly_web
cp ./config.json ./release/config.json
zip -rm release.zip release
