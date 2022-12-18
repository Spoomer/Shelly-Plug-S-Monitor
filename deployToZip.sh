#! bin/bash

cargo build -r
cd ./frontend_vue
npm run build
cd ..
mkdir release
cp -r ./wwwroot/ ./release/wwwroot/
cp ./target/release/shelly_web ./release/shelly_web
cp ./config.json ./release/config.json
zip -rm release.zip release
