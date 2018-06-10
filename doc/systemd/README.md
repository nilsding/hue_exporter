# SystemD unit

This directory contains a simple SystemD unit, `hue_exporter.service`.  To use
it, first modify the paths in the `ExecStart=` and `Environment` options to
match your environment, then copy it to
`/etc/systemd/system/hue_exporter.service`.

**Make sure you have already authenticated with the Hue bridge by running
`./bin/hue_exporter` for the first time!**

Finally, run the following commands as a privileged user (most likely `root`) to
enable `hue_exporter` at startup:

```sh
systemctl daemon-reload
systemctl enable hue_exporter
systemctl start hue_exporter
```
