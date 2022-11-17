# flfctl

A library and command-line tool to control FlashForge 3D printers

Tested on:

  - [x] FlashForge Adventurer III

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

Check printer status:
```
$ flfctl status
Status: READY
  Head: READY
 Stops: X ON / Y off / Z off
  File:
Nozzle:  25/0 °C
   Bed:  13/0 °C
```

Check the help for more commands:

```
$ flfctl --help
Control networked FlashForge 3d printers

Usage: flfctl [OPTIONS] <COMMAND>

Commands:
  scan    Scan the local network with a multicast UDP ping
  info    Get info about the printer (model, name, ...)
  status  Check printer status
  ls      List files in internal storage
  led     Turn the LED on or off
  rename  Rename the printer
  help    Print this message or the help of the given subcommand(s)

Options:
  -a, --address <ADDRESS>  Address of the printer to connect
  -h, --help               Print help information
```