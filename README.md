# Font Manager CLI
A simple CLI to manage fonts on Linux

## WIP!!

## Installation
for now, you need to compile from source, soon to be published as a crate.

## About
This utility simplifies managing fonts inside of the terminal.

Built-in support for nerd fonts on the [nerd font aggregator proyect](https://github.com/ryanoasis/nerd-fonts/)

All the fonts will be installed under the `$HOME/.fonts/` creating it if it doesn't exist

## Usage
use help to understand all options
```bash
font-manager --help
```

### Install
you can install nerd fonts that are contained on the project just by giving the name:
```bash
font-manager install --nerd FiraCode
```

or install any font giving the path to a zip file containing the fonts:
```bash
font-manager install --from-zip /path/to/zipfile/font.zip
```

If you don't want to manually download the font, you might pass the url containing the zip file

Ex: the JetbrainsMono font
```bash
font-manager install --from-url https://download.jetbrains.com/fonts/JetBrainsMono-2.242.zip
```

Installing nerd fonts won't leave any residual files, but downloading will leave the zip file, you can indicate a flag 
to delete this. (this flag also works for `--from-zip` if you want to delete the zip file as well)
```bash
font-manager install --from-url https://download.jetbrains.com/fonts/JetBrainsMono-2.242.zip --delete-zip
font-manager install --from-zip /path/to/zipfile/font.zip --delete-zip
```

### Uninstall
any installed nerd font can be removed just by giving the font name
```bash
font-manager uninstall FiraCode
```

If you wish to remove any other installed font, you need to give the exact folder name containing the fonts on the `$HOME/.fonts/` folder. 
Usually, it will use the zip name as the name
```bash
font-manager uninstall JetBrainsMono-2.242
```
