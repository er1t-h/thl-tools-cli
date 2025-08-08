# THL Tools CLI

If you're here, you're probably looking to have fun with the files of The Hundred Line - Last Defense Academy. I'll do my best to explain clearly what you should do!

## Requirements

First, you need to install Rust. Just follow the instructions on [this](https://www.rust-lang.org/fr/tools/install) site.

Then, open your terminal, and get the code of this repository, either by cloning it using `git clone https://github.com/er1t-h/thl-tools-cli.git`, or extracting [this](https://github.com/er1t-h/thl-tools-cli/archive/refs/heads/main.zip) archive.

Once that's done, enter the directory in your terminal. (if you've chosen the extracting solution, right-click in the folder, and open in terminal).

Then, you'll need to know the path to your game file. Most of the time, it should be something like `C:\Program Files (x86)\Steam\steamapps\common\The Hundred Line -Last Defense Academy-`

## What do you want to do?

### I want to get all ressources of the game.

Run `cargo x "[PATH_TO_THE_GAME_FILE]/gamedata/app_0.dx11.mvgl" extracted-app-0`  
The extracted file will be ~25Go large, which is the size of all the resources. There's no temporary file or anything. It might take quite some time.  
The sprites will be located in `extracted-app-0/images/`. If you don't see any image preview, I advise you install [paint.NET](https://www.getpaint.net/).

### I want to change some sprites

Follow the "I want to get all ressources of the game", and change any of the sprite. Be sure to save it in the `.dds` format. I would advise you to create a `modified-files` folder, with the same structure and file names.

If you're playing the game in English, run `cargo p modified-files [PATH_TO_THE_GAME_FILE]/gamedata/patch_1.dx11.mvgl`. That way, you'll only repack the files you modified, so it'll take less time. (if you're playing in Japanese, replace `patch_1` by `patch_0`, or by `2` if in Traditionnal Chinese, and `3` if in Simplified Chinese).

### I want to modify the text / create a language patch

OK, so first you're going to want a file with all the dialogues. Pretty easy!

Run `cargo xd [PATH_TO_THE_GAME_FILES] [languages_you_want_to_extract]`  
For instance, if you want the text in both Japanese and English, run `cargo xd "C:\Program Files (x86)\Steam\steamapps\common\The Hundred Line -Last Defense Academy-" japanese,english`

You should get a `full-text.csv` file, that you can open in any spreadsheet you want. Since there's like 140k lines, though, most software will probably take their time loading.
I personnaly use a self-hosted Grist, but it's outside the scope of this README.

Once you finish you modifications, you'll probably want to put your text in the game. First, assert that the first column of your `full-text.csv` are Message IDs, the 3rd column are your modifications, and the last column are file paths.

Then, run `cargo rd full-text.csv [PATH_TO_THE_GAME_FILES] [REFERENCE_LANGUAGE]`. The reference language is the language the untranslated line will be.
For instance, to apply your patch on the English translation (meaning any untranslated line will remain in English), you'll want to run something like `cargo rd full-text.csv "C:\Program Files (x86)\Steam\steamapps\common\The Hundred Line -Last Defense Academy-" english`

Tadam! You successfully applied your patch! Just run the game, and everything should be good. Note that some menu lines are only present in other, platform-specific files (like the "Close the Game" line on the title screen).

## Something doesn't work as expected

Go to the issues (at the top of this page). First, search your error, maybe someone already add it. Otherwise, open a new issue. Copy your error message, explaining what you wanted to do, and I'll try to locate the problem. (if I have the time though)
