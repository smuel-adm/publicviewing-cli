# PublicViewing-Cli

Small command line application that displays one or many given URLs
in a OS native WebView.

[![](https://github.com/smuel-adm/publicviewing-cli/actions/workflows/build.yml/badge.svg)](https://github.com/smuel-adm/publicviewing-cli/actions)
[![](https://github.com/smuel-adm/publicviewing-cli/actions/workflows/release.yml/badge.svg)](https://github.com/smuel-adm/publicviewing-cli/releases)


## Usage

Execute the application from the commandline with one URL as argument.

```
publicviewing-cli https://blog.fefe.de
```

If you give more then one URL as argument the URL's are cycled each 10 seconds.

This can be tuned with the `--cycle-sec` parameter.

```
publicviewing-cli https://blog.fefe.de https://zzeroo.com https://bbc.co.uk --cycle-sec 2
```


The parameters `--fullscreen` or `--maximized--maximized` can be used to control the window start behavior.

To send the window to a specific monitor screen use the `--monitor` paramter.


![](res/publicviewing-running.gif)

Use command line argument `--help` for description of all parameters.

![](res/command-line-example.png)


## Installation

Visit the projects [releases] page and download the packaged application that meets your OS.

Extract the archive, in the subfolder `publicviewing-cli` you'll find the application `publicviewing-cli` or `publicviewing-cl.exe`, depends on your operation system.

Execute the application direct or put them in a PATH enabled location.

## Dependencies

### Windows (maybe optional)

**Microsoft Visual C++ Redistributable** must be installed on the target systems

[https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist]

On older systems it might be also necessary to install Microsoft's WebView2 Runtime.

[https://developer.microsoft.com/de-de/microsoft-edge/webview2]


## License

**GPL-3.0-or-later**


[releases]: https://github.com/smuel-adm/publicviewing-cli/releases