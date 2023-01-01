/*
 * @Author: ZuoXichen
 * @Date: 2022-10-26 19:41:43
 * @LastEditTime: 2022-12-28 19:33:24
 * @LastEditors: ZuoXichen
 * @Description: 
 */
mod parser_test {
    use utils::release_utils::ReleasePraser;
    use java_remote;
    #[test]
    fn parser_test(){
        let paths = "./";
        let parser = ReleasePraser::new(paths);
        let dic  = praser::prase();
        println!("{}", dic);
    }
    #[test]
    fn json_parser(){
        let json = std::fs::File::open("./releases.json");
        let result:Vec<JsonInfo> = serde_json::from_str(json).unwrap();
    }


}