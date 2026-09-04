
# KOReader progress sync server

The API is compatible with `koreader-sync-server`, but instead of Redis, `kosync` uses `sled`.

- bin linux-musl-static 2.1MB
- docker image compressed 4.11MB

```bash
KOSYNC_ADDR=0.0.0.0:3000 ./kosync
```

## docker

```bash
docker pull lzyorstudio/kosync
```

## build

for linux

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

for docker

To build for other platforms, modify the TARGET_ARCH variable in ./docker/make.sh.

For example, with Raspbarry Pi 4, use TARGET_ARCH=aarch64-unknown-linux-gnu
```bash
./docker/make.sh
```

## WIP
