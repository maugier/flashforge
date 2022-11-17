# flfctl

A library and command-line tool to control FlashForge 3D printers

Tested on:

 [X] FlashForge Adventurer III

## Usage

Scan for printers on the local network:

```
$ flfctl scan
192.168.1.xxx    My 3D Printer
192.168.1.yyy    My Other 3D Printer
$
```

Set the target printer address in your environment:
```
export FLFCTL_ADDRESS="192.168.1.xxx:9988"
```

See `flfctl --help` for more commands.