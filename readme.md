# Description

- Dashboard for Shelly Plug S
- Chart
- Cost Calculation
- optional energy archive
<img width="552" height="776" alt="grafik" src="https://github.com/user-attachments/assets/97945c87-9e86-4f6e-b923-17f2fca2033a" />


# Install (Linux)
- requirement: podman or docker
- clone repository
- run deploy_local.sh in Linux or deploy_local.bat in windows
- you will find a release folder in the root folder


# config.json
- host = ip address where the webserver should run ( e.g. 127.0.0.1 or 0.0.0.0)
- port = desired port for webapp
- archive = size for archive in kB (`null` for no archive and `0` for unlimited size)
- authentication = [basic authentication](https://de.wikipedia.org/wiki/HTTP-Authentifizierung#Basic_Authentication) for Shelly Plug S, if enabled (`null` if disabled)
- shellyApiUrl = address of shelly plug s


# Cross-Compile to raspberry pi zero
## Install rust
https://www.rust-lang.org/learn/get-started
## Install cross
`cargo install cross --git https://github.com/cross-rs/cross`
## Cross-Compile
`cross build --release --target arm-unknown-linux-gnueabihf`

...or use deploy_to_zip.sh
