#! /bin/sh
cd ..
container_runtime="podman"
command -v docker >/dev/null 2>&1 && container_runtime="docker"
$container_runtime run --rm -v "$PWD":/usr/src/app -w /usr/src/app/ rust cargo build
$container_runtime run --rm -v "$PWD":/usr/src/app -w /usr/src/app/frontend_vue node npm install
$container_runtime run --rm -v "$PWD":/usr/src/app -w /usr/src/app/frontend_vue node npm run build

final_folder=${1:-release}
mkdir $final_folder
cp -r ./wwwroot/ ./$final_folder/
cp ./target/debug/shelly_web ./$final_folder/shelly_web
cp ./config.json ./$final_folder/config.json