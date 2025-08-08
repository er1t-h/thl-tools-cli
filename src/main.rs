use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result};
use clap::Parser;
use cli::{Action, CliArgs};
use tempfile::{NamedTempFile, TempDir};
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
            let patched_languages = languages
                .iter()
                .map(|_| NamedTempFile::new().expect("Couldn't create temp file"))
                .collect::<Vec<_>>();
            for (language, patched_archive) in languages.iter().zip(&patched_languages) {
                let dir = TempDir::new().expect("couldn't create temp dir");
                Extractor::new()
                    .extract(
                        &mut BufReader::new(File::open(game_data.join(language.text_file_name()))?),
                        dir.path(),
                    )
                    .with_context(|| {
                        format!("error during extraction of {}", language.text_file_name())
                    })?;
                Extractor::new()
                    .with_overwrite(true)
                    .extract(
                        &mut BufReader::new(File::open(
                            game_data.join(language.patch_file_name()),
                        )?),
                        dir.path(),
                    )
                    .with_context(|| {
                        format!("error during extraction of {}", language.patch_file_name())
                    })?;
                Packer::new()
                    .pack(dir.path(), &mut BufWriter::new(patched_archive))
                    .with_context(|| {
                        format!("error while repacking language {}", language.name())
                    })?;
            }
            DialogueExtractor::new()
                .extract(
                    &patched_languages
                        .iter()
                        .zip(&languages)
                        .map(|(file, language)| (file, language.name()))
                        .collect::<Vec<_>>(),
                    &mut BufWriter::new(File::create(&destination)?),
                )
                .context("error while extracting dialogues")?;
        }
        Action::ExtractDialoguesRawPath {
            file_paths: file_path,
            destination,
            ..
        } => {
            DialogueExtractor::new()
                .extract(
                    &file_path
                        .into_iter()
                        .enumerate()
                        .map(|(i, x)| (x, format!("LANG {i}")))
                        .collect::<Vec<_>>(),
                    &mut BufWriter::new(File::create(&destination)?),
                )
                .context("error while extracting dialogues")?;
        }
        Action::RepackDialogues {
            full_text,
            reference_language,
            game_path,
            cleanup,
        } => {
            let game_path = game_path.join("gamedata");
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let backed_text = game_path.join(format!(
                "{}.{}",
                reference_language.text_file_name(),
                timestamp
            ));

            let backed_patch = game_path.join(format!(
                "{}.{}",
                reference_language.patch_file_name(),
                timestamp
            ));

            std::fs::rename(
                game_path.join(reference_language.text_file_name()),
                &backed_text,
            )?;
            std::fs::rename(
                game_path.join(reference_language.patch_file_name()),
                &backed_patch,
            )?;

            DialogueRepacker::new().repack(
                &mut BufReader::new(File::open(&full_text)?),
                &mut BufReader::new(File::open(&backed_text)?),
                &mut BufWriter::new(File::create(
                    game_path.join(reference_language.text_file_name()),
                )?),
            )?;

            DialogueRepacker::new().repack(
                &mut BufReader::new(File::open(&full_text)?),
                &mut BufReader::new(File::open(&backed_patch)?),
                &mut BufWriter::new(File::create(
                    game_path.join(reference_language.patch_file_name()),
                )?),
            )?;

            if cleanup {
                std::fs::remove_file(backed_text)?;
                std::fs::remove_file(backed_patch)?;
            }
        }
        Action::RepackDialoguesRaw {
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
