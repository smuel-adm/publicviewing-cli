# PublicViewing-Cli

Small command line application that displays one or many given URLs
in a OS native WebView.

[![](https://github.com/smuel-adm/publicviewing-cli/actions/workflows/build.yml/badge.svg)](https://github.com/smuel-adm/publicviewing-cli/actions)
[![](https://github.com/smuel-adm/publicviewing-cli/actions/workflows/release.yml/badge.svg)](https://github.com/smuel-adm/publicviewing-cli/releases)


## Usage

The following example executes the application with one URL as argument.

```
publicviewing-cli https://blog.fefe.de
```

If you give more then one URL as argument the URL's are cycled each 10 seconds.
The cycle time can be tuned with the `--cycle-sec` parameter.

```
publicviewing-cli https://blog.fefe.de https://zzeroo.com https://bbc.co.uk --cycle-sec 2
```


The parameters `--fullscreen` or `--maximized` can be used to control the window start behavior.
To send the window to a specific monitor screen use the `--monitor` paramter.

![](res/publicviewing-running.gif)

Use command line argument `--help` for description of all parameters.

```
Usage: publicviewing-cli.exe [OPTIONS] [URLS]...

Arguments:
  [URLS]...  Optional url(s) to open, space separated [default: https://google.com]

Options:
  -a, --above                  window will always be above other windows
  -c, --cycle-sec <CYCLE_SEC>  cycle time between site reloads
                                   if more then one URL was given
                                   these URL's are cycled after that time [default: 10]
  -f, --fullscreen             open window in fullscreen
  -m, --maximized              open window maximized
      --monitor <MONITOR>      monitor number on which the window should open
                                   This has no effect if you have only one monitor!
                                   Android / Linux(Wayland): Unsupported
  -h, --help                   Print help
  -V, --version                Print version
  ```

## Installation

Visit the projects [releases] page and download the packaged application that meets your OS.
Extract the archive, in the subfolder `publicviewing-cli` you'll find the application `publicviewing-cli` or `publicviewing-cl.exe`, depends on your operation system.
Then execute the application or put them in a PATH enabled location.

## Dependencies

### Windows (maybe optional)

**Microsoft Visual C++ Redistributable** must be installed on the target systems

[https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist]

On older systems it might be also necessary to install Microsoft's WebView2 Runtime.

[https://developer.microsoft.com/de-de/microsoft-edge/webview2]


## License

**GPL-3.0-or-later**


[releases]: https://github.com/smuel-adm/publicviewing-cli/releases