#! /bin/sh
cd ..
cargo build
podman run --rm -v "$PWD":/usr/src/app -w /usr/src/app/frontend_vue node npm install
podman run --rm -v "$PWD":/usr/src/app -w /usr/src/app/frontend_vue node npm run build

final_folder=release
mkdir $final_folder
cp -r ./wwwroot/ ./$final_folder/
cp ./target/debug/shelly_web ./$final_folder/shelly_web
cp ./config.json ./$final_folder/config.json
