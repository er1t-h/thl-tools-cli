# THL Tools

CLI tool to extract and repack files from the "The Hundred Line" game

## Usage

You need to have Rust installed on your computer. Due to the difference of path formatting, I don't know if this program works on Windows.

### Extraction

To extract the content of a `.mvgl` archive:
```sh
cargo x path/to/the/archive.mvgl path/to/the/extracted/directory
```

### Packing

To repack the content of a folder to a `.mvgl` archive:
```sh
cargo p path/to/the/directory path/to/the/created/archive
```

### Extract all dialogues in a single file

If you ever want to create a patch for the game, you'll most likely want to have all the dialogues of the game in one big file.
```
cargo xd path/to/the/game/directory languages
```
Where the game directory will most likely be `"/mnt/c/Program Files (x86)/Steam/steamapps/common/The Hundred Line -Last Defense Academy-"` if you're under WSL, and languages being a string of comma-separated languages (e.g. japanese,english).
There might be a bug where inputting only one language would panic, just put another language and you should be good for now.

### Repack the dialogues

Once you translated the game, you could want to put the lines back in the files. If it's the case, take the CSV you got by extracting the text of the game.
It MUST have:
- The message ID as the 1st column
- The translated dialogue as the 3rd column
- The name of the file as the LAST column

(just keep the structure outputted by the extract-dialogues command and you should be good.)

Then run
```
cargo rd path/to/the/dialogues/csv/file path/to/the/reference/mvgl path/to/the/destination
```
The reference mvgl will probably be: `/mnt/c/Program Files (x86)/Steam/steamapps/common/The Hundred Line -Last Defense Academy-/gamedata/app_text[ID].dx11.mvgl`
Where ID is:
- 0 for Japanese
- 1 for English
- 2 for Traditional Chinese
- 3 for Simplified Chinese

## Important note for translators

If you plan to translate the game, the dialogues are actually split between `gamedata/app_text[ID].dx11.mvgl` and `gamedata/patch_app_text[ID].dx11.mvgl`.
I'll probably find a better way to handle this at one point, but if you want to extract the `patch` one, you can use the `extract-dialogues-raw-path` for now.
