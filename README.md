# Temp Linker

Create a link of a hwmon temp input based on name and label.

## How to Build

```shell
nix build .\#packages.x86_64-linux.default
```

## How to Run

```shell
./result/bin/temp-linker --name k10temp --link-path /tmp/temperature
```

Specify an additional label that needs to match.

```shell
./result/bin/temp-linker --name k10temp --link-path /tmp/temperature --label Tctl
```

## Get Name and Label

```bash
for i in /sys/class/hwmon/hwmon*/temp*_input; do echo "$(<$(dirname $i)/name): $(cat ${i%_*}_label 2>/dev/null || echo no_label) $i"; done
```
