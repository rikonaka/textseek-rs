use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;

const DEFAULT_FILE_EXTENSION: [&str; 32] = [
    "doc", "docm", "docx", "dps", "dpt", "eid", "eip", "eis", "epub", "et", "ett", "htm", "html",
    "hwp", "md", "mht", "mhtml", "odp", "ods", "odt", "ofd", "pdf", "ppt", "pptm", "pptx", "rtf",
    "txt", "wps", "wpt", "xls", "xlsm", "xlsx",
];

/// Simple program to search file contain some content
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file extension you wanna search
    #[arg(short, long, default_value = "")]
    extension: String,

    /// The target search path
    #[arg(short, long, default_value = "./")]
    target: String,
}

fn visit_dirs<P: AsRef<Path>, T: AsRef<[String]>>(
    target_dir: P,
    file_extension: T,
) -> io::Result<Vec<String>> {
    let mut ret = Vec::new();
    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let tmp = visit_dirs(&path, &file_extension)?;
            ret.extend_from_slice(&tmp);
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if let Some(ext) = ext.to_str() {
                    let fe = file_extension.as_ref();
                    if fe.contains(&ext.to_string()) {
                        ret.push(ext.to_string());
                    }
                };
            };
            println!("Found file: {}", path.display());
        }
    }
    Ok(ret)
}

fn extension_parser<P: AsRef<str>>(extension: P) -> Vec<String> {
    let ext = extension.as_ref();
    if ext.len() > 0 {
        if ext.contains(",") {
            let ext_split: Vec<String> = ext.split(",").map(|x| x.trim().to_string()).collect();
            ext_split
        } else {
            vec![ext.to_string()]
        }
    } else {
        let default_ext: Vec<String> = DEFAULT_FILE_EXTENSION
            .iter()
            .map(|x| x.to_string())
            .collect();
        default_ext
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let extension = extension_parser(args.extension);

    let dir_path = "./target";
    visit_dirs(dir_path, &extension);
    Ok(())
}
