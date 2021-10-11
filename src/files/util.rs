use super::ExtractOptions;
use std::path::Path;

const VALID_OPTIONS: [&str; 8] = ["Y", "y", "Yes", "yes", "N", "n", "No", "no"];
const AFFIRMATIVE_OPTIONS: [&str; 4] = ["Y", "y", "Yes", "yes"];

pub(super) fn should_ignore(opts: ExtractOptions, file_name: &str, outpath: &Path) -> bool {
    log::debug!("file name: {}", file_name);
    // Safe to unwrap as the path was already contructed and exists
    // Ignores Windows fonts
    if outpath.to_str().unwrap().contains("Windows") {
        return true;
    }

    if opts.interactive {
        println!();
        log::info!("Install: {}?", file_name);
        let mut selection = String::from("");
        while !VALID_OPTIONS.contains(&selection.as_str()) {
            log::info!("[Yes/No]");
            selection = text_io::read!();
        }
        if AFFIRMATIVE_OPTIONS.contains(&selection.as_str()) {
            return false;
        }
        return true;
    }

    match opts.use_otf {
        true => file_name.ends_with(".ttf"),
        false => file_name.ends_with(".otf"),
    }
}
