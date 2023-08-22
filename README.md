
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

```bash
./docker/make.sh
```

## TLS

Generate self-signed certificates:

```bash
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:secp384r1 -days 3650 -nodes -keyout key.pem -out cert.pem
```

## Systemd

For non-Docker deployment, a systemd unit file and its matching env file are available in [contrib](contrib/).

## WIP
