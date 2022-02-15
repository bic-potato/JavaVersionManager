use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use console::Style;
use crate::java_ver::JavaNew;

pub struct ReleaseParser
{
    path: String
}

impl ReleaseParser {
    pub fn new(path: &str) -> ReleaseParser {
        let paths = String::from(path);
        return ReleaseParser{path:paths};
    }

    pub fn parse(&self) -> HashMap<String, String> {
        let mut release = PathBuf::from(&self.path);
        release.push("release");
        let red = Style::new().red();
        let error = format!("{} Not A Valid Release", red.apply_to("Error"));
        let mut release_file = File::open(&release).expect(&error);
        let mut content = String::from("");
        release_file.read_to_string(&mut content).unwrap();
        let vector :Vec<&str> = content.split("\n").collect();
        let mut dic: HashMap<String, String> = HashMap::new();
        for element in vector {
            let vec:Vec<&str> = element.split("=").collect();
            println!("{:?}", vec);
            if vec.len() > 1 {
                let a = vec[0];
                let mut b = vec[1];
                let b1 = b.replace("\"", "");
                let mut b2 = String::from("");
                if b1.contains("\r") {
                    b2 = b1.replace("\r", "");
                } else {
                    b2 = b1;
                }
                dic.insert(a.to_owned(), b2);
            }
        }
        let dics = dic.to_owned();
        return dics;
    }
}

#[cfg(test)]
mod test {
    use crate::utils::release_utils::ReleaseParser;
    #[test]
    fn parser_test() {
        let paths = "./test/";
        let mut parser = ReleaseParser::new(paths);
        let dic = parser.parse();
        println!("{:?}", dic);
    }
}