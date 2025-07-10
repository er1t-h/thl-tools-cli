use std::{borrow::Cow, path::PathBuf};

use anyhow::{Ok, Result, bail};
use regex::Regex;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Language {
    English,
    Japanese,
    SimplifiedChinese,
    TraditionalChinese,
}

impl Language {
    pub fn name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Japanese => "Japanese",
            Self::TraditionalChinese => "Traditional Chinese",
            Self::SimplifiedChinese => "Simplified Chinese",
        }
    }

    pub fn text_file_name(self) -> &'static str {
        match self {
            Self::Japanese => "app_text00.dx11.mvgl",
            Self::English => "app_text01.dx11.mvgl",
            Self::TraditionalChinese => "app_text02.dx11.mvgl",
            Self::SimplifiedChinese => "app_text03.dx11.mvgl",
        }
    }
}

fn get_default_csv_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("full-text.csv");
    path
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Action {
    /// Extracts a `.mvgl` archive to specified folder.
    Extract {
        /// The path to the `.mvgl` archive.
        source: PathBuf,
        /// The path to the folder to create.
        destination: PathBuf,
        /// If true, will not rename `.img` files into `.dds` files.
        #[arg(long)]
        no_rename_images: bool,
        /// Regex that will determine which files to extract.
        #[arg(long)]
        extract_only: Option<Regex>,
        /// If true, will replace files with matching name in the destination folder
        #[arg(long)]
        overwrite: bool,
        /// If true, files will be extracted with a single thread
        #[arg(long)]
        no_multi_threading: bool,
    },
    /// Packs a folder into a `.mvgl` archive.
    Pack {
        /// The path to the folder.
        source: PathBuf,
        /// The path to the `.mvgl` archive to create.
        destination: PathBuf,
        /// If true, will overwrite the `destination` file if it exists.
        #[arg(long)]
        overwrite: bool,
        /// If true, will not rename `.dds` files into `.img` files.
        #[arg(long)]
        no_rename_images: bool,
    },
    /// Read every lines of a `.mbe` file.
    ReadLines {
        /// The path to the `.mbe` file.
        source: PathBuf,
        /// The prefix appended at the beginning of each line.
        #[arg(short, long, default_value_t = Cow::Borrowed("> "))]
        prefix: Cow<'static, str>,
    },
    /// Extract all dialogues from the game, putting them all into a single `.csv`
    ExtractDialogues {
        /// The path to the game directory.
        ///
        /// Usually, something like 'C:\Program Files (x86)\Steam\steamapps\common\The Hundred Line -Last Defense Academy-'
        game_path: PathBuf,
        /// The languages that will be exported to the `.csv`, comma separated.
        ///
        /// If you want to export Japanese and English, use 'japanese,english'.
        #[arg(value_delimiter = ',')]
        languages: Vec<Language>,
        /// The path to the destination.
        #[arg(long, default_value=get_default_csv_path().into_os_string())]
        destination: PathBuf,
        /// If true, will overwrite the `destination` file if it exists.
        #[arg(long)]
        overwrite: bool,
    },
    /// Like ExtractDialogues, but with the path to the mvgl
    ExtractDialoguesRawPath {
        /// The path to the game directory.
        ///
        /// Usually, something like 'C:\Program Files (x86)\Steam\steamapps\common\The Hundred Line -Last Defense Academy-'
        file_path_1: PathBuf,
        /// The path to the game directory.
        ///
        /// Usually, something like 'C:\Program Files (x86)\Steam\steamapps\common\The Hundred Line -Last Defense Academy-'
        file_path_2: PathBuf,
        /// The path to the destination.
        #[arg(long, default_value=get_default_csv_path().into_os_string())]
        destination: PathBuf,
        /// If true, will overwrite the `destination` file if it exists.
        #[arg(long)]
        overwrite: bool,
    },
    /// Repacks all dialogues in a single `.mvgl` file.
    RepackDialogues {
        /// The path to the `.csv` containing all text.
        full_text: PathBuf,
        /// The `.mvgl` to use to repack.
        ///
        /// The dialogues of the game are stored in GAME_PATH/gamedata/app_text0[LANGUAGE].dx11.mvgl.
        /// Where Japanese is LANGUAGE = 0, English is LANGUAGE = 1, Traditional Chinese is
        /// LANGUAGE = 2 and Simplified Chinese is LANGUAGE = 3
        reference_mvgl: PathBuf,
        /// The path to the repacked text
        destination: PathBuf,
        /// If true, will overwrite the `destination` file if it exists.
        #[arg(long)]
        overwrite: bool,
    },
    CheckMbes {
        path: PathBuf,
    },
}

impl CliArgs {
    pub fn validate(&self) -> Result<()> {
        match &self.action {
            Action::Extract {
                source,
                destination,
                no_rename_images: _,
                extract_only: _,
                overwrite: _,
                no_multi_threading: _,
            } => {
                if !source.is_file() {
                    bail!("{} should be a valid file", source.display());
                }
                if destination.exists() && !destination.is_dir() {
                    bail!("{} should not exist", destination.display());
                }
            }
            Action::Pack {
                source,
                destination,
                overwrite,
                no_rename_images: _,
            } => {
                if !source.is_dir() {
                    bail!("{} should be a valid directory", source.display())
                }
                if !*overwrite && destination.exists() {
                    bail!("{} should not exist", destination.display())
                }
            }
            Action::ReadLines { source, .. } => {
                if !source.is_file() {
                    bail!("{} should be a valid file", source.display())
                }
            }
            Action::ExtractDialogues {
                game_path,
                languages,
                destination,
                overwrite,
            } => {
                if !game_path.is_dir() {
                    bail!("{} should be a valid directory", game_path.display());
                }
                if languages.is_empty() {
                    bail!("at least one language should be selected");
                }
                if !*overwrite && destination.exists() {
                    bail!("{} should not exist", destination.display());
                }
            }
            Action::ExtractDialoguesRawPath {
                destination,
                overwrite: _,
                file_path_1,
                file_path_2,
            } => {
                if !file_path_1.is_file() {
                    bail!("{} should be a valid file", file_path_1.display());
                }
                if !file_path_2.is_file() {
                    bail!("{} should be a valid file", file_path_2.display());
                }
                if destination.exists() && !destination.is_dir() {
                    bail!("{} should not exist", destination.display());
                }
            }
            Action::RepackDialogues {
                full_text,
                reference_mvgl,
                destination,
                overwrite,
            } => {
                if !full_text.exists() {
                    bail!("{} should exist", full_text.display());
                }
                if !reference_mvgl.exists() {
                    bail!("{} should exist", reference_mvgl.display());
                }
                if !*overwrite && destination.exists() {
                    bail!("{} shouldn't exist", destination.display());
                }
            }
            Action::CheckMbes { path } => {
                if !path.is_dir() {
                    bail!("{} should be a valid directory", path.display());
                }
            }
        }
        Ok(())
    }
}

///
/// thl-tools: A CLI tool to extract and repack files from the "The Hundred Line" game
///
#[derive(Debug, Clone, clap::Parser)]
pub struct CliArgs {
    /// The subcommand to use
    #[command(subcommand)]
    pub action: Action,
}
