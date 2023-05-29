# Description

- Dashboard for Shelly Plug S
- Chart
- Cost Calculation
- optional energy archive
# config.json

- port = desired port for webapp
- archive = size for archive in kB (`null` for no archive and `0` for unlimited size)
- authentication = [basic authentication](https://de.wikipedia.org/wiki/HTTP-Authentifizierung#Basic_Authentication) for Shelly Plug S, if enabled (`null` if disabled)


# Cross-Compile to raspberry pi zero

## Install linker
- rustup target add arm-unknown-linux-gnueabihf
- Download gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf from https://developer.arm.com/downloads/-/gnu-a
- change path in ./.cargo/config.toml
## Cross-Compile
`cargo build -r --target arm-unknown-linux-gnueabihf`