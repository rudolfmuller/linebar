# linebar

<b>A tool for customizing the status bar appearance on <i>Wayland (Sway)</i>
<br>Unlike other bars, it doesn't have its own configuration folder, but only a single file in the sway folder</b>
<br><br>
POSIX focused - Only
 - <b>Linux: </b>Supported <br>
 - <b>macOS: </b>Supported (untested)<br>
 - <b>Windows: </b>Untested<br>
<br>

### <b>PLACEHOLDERS :</b>

| Key               | Description                              |
|-------------------|------------------------------------------|
| `disk.free`       | <b>free disk space</b>                   |
| `disk.used`       | <b>used disk space</b>                   |
| `cpu.used`        | <b>CPU usage percentage</b>              |
| `memory.used`     | <b>used memory</b>                       |
| `memory.free`     | <b>free memory</b>                       |
| `date.year`       | <b>full year (e.g. 2026)</b>             |
| `date.year.short` | <b>last two digits of year (e.g. 26)</b> |
| `date.month`      | <b>month number (01–12)</b>              |
| `date.day`        | <b>day of month (01–31)</b>              |
| `time.hour`       | <b>hour (00–23)</b>                      |
| `time.min`        | <b>minute (00–59)</b>                    |
| `time.sec`        | <b>second (00–59)</b>                    |

<br>

### <b>HOW TO CONFIGURE:</b>
make file `~/.config/sway/linebar.toml`,
and write into e.g.:<br>
```
[general]
interval = 1000
format = "[{date.day}-{date.month}.{date.year.short} {time.hour}:{time.min}:{time.sec}]"
```

Change sway config file `sway/config`<br>
`status_command <...>` to `status_command linebar`

