# Temp Linker

Create a link of a hwmon temp input based on name and label.

## Get Name and Label

```bash
for i in /sys/class/hwmon/hwmon*/temp*_input; do echo "$(<$(dirname $i)/name): $(cat ${i%_*}_label 2>/dev/null || echo no_label) $i"; done
```
