use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::Command;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let out_name;

    // if rust project
    if read_dir(".").unwrap()
        .map(|x| x.unwrap().file_name())
        .any(|x| x == "Cargo.toml") {
        println!("Cargo project detected");


        let release_dir = Path::new("target").join("release");
        let usr_bins = get_usr_bin()?;

        // ensure build target exists
        let status = Command::new("cargo").args(["build", "--release"]).status()?;
        println!("Cargo project build w/ status {}", status);

        // get binary targets
        let bins = get_binaries(&release_dir)?;

        // ensure only 1 was created
        if args.len() > 1 { out_name = &args[1]; }
        else { out_name = &bins[0] }
        if bins.len() != 1 { return Err("Incorrect number of binaries within release target".into())}
        println!("Single target exists");


        // copy file
        let i_path = &release_dir.join(&bins[0]);
        let o_path =  &usr_bins.join(out_name);
        fs::copy(i_path, o_path)?;
        println!("{} copied to {}", i_path.to_str().unwrap(), o_path.to_str().unwrap());
    }


    Ok(())
}

fn get_binaries(release_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_dir(&release_dir).unwrap()
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_file())
        .filter_map(|x| match x.extension() {
            Some(_) => None,
            None => Some(x),
        })
        .map(|x| x.file_name().unwrap().to_str().unwrap().to_owned())
        .filter(|x| !x.starts_with("."))
        .collect()
    )
}

fn get_usr_bin() -> Result<PathBuf, Box<dyn Error>> {
    let usr_bin = Path::new("/usr").join("local").join("bin");
    if !usr_bin.exists() || !usr_bin.is_dir() {
        fs::create_dir(&usr_bin)?
    }
    Ok(usr_bin)
}