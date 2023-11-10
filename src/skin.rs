use std::{
    fs::{self},
    io::{self, Write},
    path::{Path, PathBuf},
};

use ini::Ini;
use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::ASSETS;

pub struct Skin {
    pub location: PathBuf,
    location_str: String,
    hit_circle_prefix: String,
}

impl Skin {
    pub fn new(location: String) -> Skin {
        let hit_circle_prefix = match Ini::load_from_file(Path::new(&location).join("skin.ini")) {
            Ok(ini) => match ini.section(Some("Fonts")) {
                Some(s) => match s.get("HitCirclePrefix") {
                    Some(s) => s.to_string(),
                    None => "default".to_string(),
                },

                None => "default".to_string(),
            },

            Err(_) => "default".to_string(),
        };

        Skin {
            location: PathBuf::from(&location),
            location_str: location,
            hit_circle_prefix,
        }
    }

    pub fn create_new_skin(&self) -> io::Result<Skin> {
        let old_loc = Path::new(&self.location);
        let new_loc_str = format!("{} - flipped", self.location_str);
        let new_loc = PathBuf::from(&new_loc_str);

        if new_loc.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Destination directory already exists",
            ));
        }

        copy_dir(old_loc, &new_loc)?;

        Ok(Skin::new(new_loc_str))
    }

    pub fn flip_default_numbers(&self) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..10 {
            for j in 0..2 {
                let elem = match j {
                    0 => format!("{}-{}.png", self.hit_circle_prefix, i),
                    1 => format!("{}-{}@2x.png", self.hit_circle_prefix, i),
                    _ => unreachable!(),
                };

                let path = self.location.join(&elem);

                let image = match image::open(&path) {
                    Ok(i) => i,
                    Err(_) => {
                        if j == 1 {
                            continue;
                        }

                        let elem_bytes = ASSETS
                            .get_file(format!("default-{}.png", i))
                            .unwrap() // unwrap because this should always be possible unless compiled by a fool
                            .contents();

                        image::load_from_memory(elem_bytes)?
                    }
                };

                let image = image.flipv().fliph();
                image.save(path)?;
            }
        }

        Ok(())
    }

    pub fn into_osk(&self) -> Result<String, Box<dyn std::error::Error>> {
        let file_name = format!("{}.osk", self.location_str);
        let mut zip = zip::ZipWriter::new(fs::File::create(&file_name).unwrap());

        let opts = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        for entry in WalkDir::new(&self.location) {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(&self.location)?;

            let name_str = match name.to_str() {
                Some(s) => s,
                None => continue,
            };

            if path.is_file() {
                zip.start_file(name_str, opts)?;
                zip.write(&fs::read(path)?)?;
            }
        }

        zip.finish()?;

        Ok(file_name)
    }
}

fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    if !to.exists() {
        fs::create_dir(to)?;
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let file_name = from.file_name().unwrap();
        let to = to.join(file_name);

        if ty.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}
