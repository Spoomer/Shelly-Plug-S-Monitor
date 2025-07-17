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

## Install cross
`cargo install cross --git https://github.com/cross-rs/cross`
## Cross-Compile
`cross build --release --target arm-unknown-linux-gnueabihf`