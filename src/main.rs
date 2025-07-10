use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::{Context, Ok, Result};
use clap::Parser;
use cli::{Action, CliArgs};
use thl_tools::{
    csv::{extract_dialogues::DialogueExtractor, repack_dialogues::DialogueRepacker},
    helpers::offset_wrapper::OffsetReadWrapper,
    mbe::MBEFile,
    mvgl::{Extractor, Packer},
};
use walkdir::WalkDir;

mod cli;

fn main() -> Result<()> {
    env_logger::init();
    let args = CliArgs::parse();
    args.validate()?;
    match args.action {
        Action::Extract {
            source,
            destination,
            no_rename_images,
            extract_only,
            overwrite,
            no_multi_threading,
        } => {
            Extractor::new()
                .with_rename_images(!no_rename_images)
                .with_multi_threading(!no_multi_threading)
                .with_name_matcher(extract_only)
                .with_overwrite(overwrite)
                .extract(&mut BufReader::new(File::open(&source)?), &destination)
                .context("something went wrong during the extraction")?;
        }
        Action::Pack {
            source,
            destination,
            no_rename_images,
            ..
        } => Packer::new()
            .with_rename_images(!no_rename_images)
            .pack(
                &source,
                &mut BufWriter::new(
                    File::create(&destination)
                        .with_context(|| format!("couldn't create {}", destination.display()))?,
                ),
            )
            .context("something went wrong during the repacking")?,
        Action::ExtractDialogues {
            game_path,
            languages,
            destination,
            ..
        } => {
            let game_data = game_path.join("gamedata");
            DialogueExtractor::new()
                .extract(
                    &languages
                        .iter()
                        .map(|x| (game_data.join(x.text_file_name()), x.name()))
                        .collect::<Vec<_>>(),
                    &mut BufWriter::new(File::create(&destination)?),
                )
                .context("error while extracting dialogues")?;
        }
        Action::ExtractDialoguesRawPath {
            file_path_1,
            file_path_2,
            destination,
            ..
        } => {
            DialogueExtractor::new()
                .extract(
                    &[(file_path_1, "LANG 1"), (file_path_2, "LANG 2")],
                    &mut BufWriter::new(File::create(&destination)?),
                )
                .context("error while extracting dialogues")?;
        }
        Action::RepackDialogues {
            full_text,
            reference_mvgl,
            destination,
            ..
        } => DialogueRepacker::new()
            .repack(
                &mut BufReader::new(File::open(&full_text)?),
                &mut BufReader::new(File::open(&reference_mvgl)?),
                &mut BufWriter::new(File::create(&destination)?),
            )
            .context("error while repacking dialogues")?,
        Action::CheckMbes { path } => {
            for file in WalkDir::new(&path) {
                let file = file?;
                if file.path().extension() == Some(OsStr::new("mbe")) {
                    let file_path = file.path();
                    let mut file = BufReader::new(File::open(file_path)?);
                    let mut file = OffsetReadWrapper::new(&mut file);
                    MBEFile::parse(&mut file).unwrap_or_else(|x| {
                        panic!(
                            "failed to read file {} after writing 0x{:x} bytes: {x}",
                            file_path.display(),
                            file.offset()
                        )
                    });
                }
            }
        }
    }
    Ok(())
}
