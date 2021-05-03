# systemd unit

This directory contains a simple systemd unit, `hue_exporter.service`.  To use
it, first modify the path in the `ExecStart=` to match your environment, then
copy it to `/etc/systemd/system/hue_exporter.service`.

You also need to copy the `hue_exporter.sysconfig` file in this directory to
`/etc/sysconfig/hue_exporter`.  Modify it to your needs; you can get the
`HUE_TOKEN` by running `hue_exporter` for the first time.

Finally, run the following commands as a privileged user (most likely `root`) to
enable `hue_exporter` at startup:

```sh
systemctl daemon-reload
systemctl enable --now hue_exporter.service
```
