use std::{
    fs::{self, File},
    io::{Read, Result, Write},
    path::{Path, PathBuf},
};

pub fn save_pgns(pgns: &[String], folder_path: &Path) -> Result<()> {
    fs::create_dir_all(folder_path)?;

    for (i, pgn) in pgns.iter().enumerate() {
        let file_path = folder_path.join(format!("{}.txt", i));

        let mut file = File::create(&file_path)?;
        file.write_all(pgn.as_bytes())?;
    }

    println!("Successfully saved the game to a folder.");

    Ok(())
}

pub fn read_pgns(folder_path: &Path) -> Result<Vec<String>> {
    let mut entries: Vec<PathBuf> = fs::read_dir(folder_path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt")
        })
        .collect();

    entries.sort_by_key(|path| {
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .and_then(|s| s.parse::<u32>().ok())
    });

    let mut pgns = Vec::new();

    for path in entries {
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        pgns.push(content);
    }

    Ok(pgns)
}
