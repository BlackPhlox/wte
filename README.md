# What is WTE?
An all-in-one realtime editor for updating of Windows Terminal settings. This gif is showing wt-editor-cli, a prototype of wte written in node.js. The goal is to create an .exe that can run either as a cli or host a local static serve and use [json-editor](https://github.com/json-editor/json-editor) to function as a gui.

![wt-editor-cli-showcase](https://user-images.githubusercontent.com/25123512/68077919-ba2a4980-fdcc-11e9-879f-6e1fecb6bb20.gif)

|Info| |
|-------|---------|
| GitHub | [![GitHub version](https://badge.fury.io/gh/BlackPhlox%2Fwt-editor-cli.svg)](https://badge.fury.io/gh/BlackPhlox%2Fwte) |
| Twitter     | [![Twitter Follow](https://img.shields.io/twitter/follow/darkphlox?style=social)](https://twitter.com/darkphlox)      |

## Related repositories
  - [wt-editor-cli](https://github.com/BlackPhlox/wt-editor-cli) - Real-time command-line editor for Windows Terminal settings
  - [wt-editor-gui](https://github.com/BlackPhlox/wt-editor-gui) - A graphical user interface version of wt-editor-cli
  - [nateshmbhat's windows-terminal-tweaker](https://github.com/nateshmbhat/windows-terminal-tweaker) - A electron-based windows terminal editor

# Prerequisites
  - Running Windows 1903 (build >= 10.0.18362.0) or later
  - Have the latest version of Windows Terminal (Preview) installed
  - Have Node.js 12 installed or later
  - Have npm (Comes with Node.js)
# Setup

Before doing anything I will highly recommend that you backup you settings.json prior to working with this project.

When running src/setup.bat

- Generates a config.json, which locates the folder of your settings.json (which is located here: ```%appdata%\..\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\```)
- Creates a backup of your current settings (called settings.json.pre.wte.backup in the same folder)
- Prompted to allow administrative rights to powershell, this is for setting a enviroment variable to the projects folder. As it is not setup yet, then you can easily decline and run the project using ```cargo run```.

## config.json
WTE uses config.json to know what settings.json-file to edit. To debug I would recommend copying your settings.json and changing the settings_folder_path for debug perpose shown below. Default is to your folder of your settings.json

```
{ 
    "settings_folder_path":"C:/Users/USERNAME/Repos/wte/src/", 
    "asset_path":"...", 
    "questionColor":"lightgray", 
    "errorColor":"red" 
} 
```

WIP

## Additionally step(WIP)
  - If you are running using the [Linux-Subsystem](https://docs.microsoft.com/en-us/windows/wsl/install-win10) you can add `alias wte='cmd.exe /c wte'` in your `~/.bashrc` file.


# Contribution

Any contribution is appreciated, their are no formalities, just create a pull request.

### Creating Pull Requests
  Push your commit to get it back up to your fork: git push origin HEAD
  Visit Github, click handy “Pull request” button that it will make upon noticing your new branch.
