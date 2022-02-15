use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use console::Style;
use crate::java_ver::JavaNew;

pub struct ReleaseParser
{
    path: PathBuf
}

impl ReleaseParser {
    pub fn new(path: &str) -> ReleaseParser {
        let path = PathBuf::from(path);
        return ReleaseParser{path};
    }

    pub fn parse(&self) -> JavaNew {
        let mut release = PathBuf::from(&self.path);
        release.push("release");
        let red = Style::new().red();
        let error = format!("{} Not A Valid Release", red.apply_to("Error"));
        let mut release_file = File::open(&release).expect(&error);
        let mut content = String::from("");
        release_file.read_to_string(&mut content).unwrap();
        let vector :Vec<&str> = content.split("\n").collect();
        let mut dic: HashMap<&str, String> = HashMap::new();
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
                dic.insert(a, b2);
            }
        }
        let path_str = self.path.to_str().unwrap();
        let java = JavaNew::new(&dic["IMPLEMENTOR"], &dic["FULL_VERSION"], &dic["JVM_VARIANT"], &dic["IMAGE_TYPE"], path_str);

        return java;
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::utils::release_utils::ReleaseParser;
    #[test]
    fn parser_test() {
        let paths = "./test/";
        let path = Path::new("./test/");
        let mut parser = ReleaseParser::new(&paths);
        let dic = parser.parse();
        // println!("{:?}", dic);
    }
}