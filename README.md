# hue\_exporter

This is a simple Prometheus exporter for various metrics from a Philips Hue
system.

Right now it only exports metrics collected from sensor devices.

This app does obviously not belong to Philips Lighting.

## Installation

Build requirements:
- Crystal 0.24.2

Build it:

```
shards build
```

## Usage

After building, just run it:

```
./bin/hue_exporter
```

When starting `hue_exporter` for the first time, it will try to authenticate
with the Hue bridge in your network.  After that is done you can safely start
`hue_exporter` as a daemon using your system's init facilities.

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
