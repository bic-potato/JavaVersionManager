/*
 * @Author: ZuoXichen
 * @Date: 2022-02-11 18:02:31
 * @LastEditTime: 2022-05-12 23:43:47
 * @LastEditors: ZuoXichen
 * @Description: 
 */
use std;
use std::path::Path;
use std::path::PathBuf;
use indicatif::ProgressStyle;
use zip;
use indicatif::ProgressBar;
use console::Term;

pub fn extract(files: &Path, current_location: &PathBuf) {
    println!("{}", files.to_str().unwrap());

    let file = std::fs::File::open(files).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    let prograss = ProgressBar::new(archive.len().try_into().unwrap());
    prograss.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    .progress_chars("#>-"));
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let str_output = outpath.to_str().unwrap();
        let str_current_location = current_location.to_str().unwrap();
        let string_cl = String::from(str_current_location);
        let out = string_cl + str_output;
        outpath = std::path::PathBuf::from(out);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                let outline = format!("File {} comment: {}", i, comment);
                prograss.println(&outline);
            }
        }

        if (*file.name()).ends_with('/') {
            let outline = format!("File {} extracted to \"{}\"", i, outpath.display());
            prograss.println(&outline);
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            let outline = format!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            prograss.println(&outline);
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
        prograss.inc(1)
    }
    prograss.finish();
}
