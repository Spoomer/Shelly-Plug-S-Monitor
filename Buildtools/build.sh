#! /usr/bin/env bash
cd ..
cd ./frontend_vue || exit
npm run build
cd ..
cargo build