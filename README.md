# hue\_exporter

This is a simple Prometheus exporter for various metrics from a Philips Hue
system.

Right now it only exports metrics collected from sensor devices and lights.

This app does obviously not belong to Philips Lighting.

## Installation

Build requirements:
- Rust

Build it:

```
cargo build --release
```

## Usage

After building, just run it:

```
./target/release/hue_exporter
```

When starting `hue_exporter` without a `HUE_TOKEN` set, it will try to
authenticate with the Hue bridge in your network.  After that is done you can
export the received token using the `HUE_TOKEN` environment variable and start
`hue_exporter` as a [daemon using e.g. systemd][systemd_doc].

### Configuration

Configuration is done via the following environment variables:

- `HUE_TOKEN`: set this to the token received from authorizing with the Hue
  bridge.  If unset the authorisation flow will start.
- `HUE_BRIDGE_URL`: set this to the URL of the Hue bridge.
  Default value: `http://hue-bridge.local`
- `BIND_ADDR`: set this to the address+port to bind to.
  Default value: `127.0.0.1:9369`

### Adding it to Prometheus

Add the following lines to the Prometheus configuration:

```yaml
scrape_configs:
  # [...] other configs may be here
  
  - job_name: 'hue_exporter'

    scrape_interval: 1s
    scrape_timeout: 1s

    static_configs:
      - targets: ['localhost:9369']
```

### Available metrics

To see the metrics exported by `hue_exporter`, just open your favourite web
browser and point it to `http://localhost:9369/metrics` (or wherever your
`hue_exporter` application is running).

## Development

TODO: Write development instructions here

## Contributing

1. Fork it ( https://github.com/nilsding/hue_exporter/fork )
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create a new Pull Request

## Contributors

- [nilsding](https://github.com/nilsding) Georg Gadinger - creator, maintainer

[systemd_doc]: ./doc/systemd/README.md
