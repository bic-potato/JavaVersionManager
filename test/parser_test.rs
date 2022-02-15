mod parser_test {
    use jvman::utils::release_utils::ReleasePraser;
    #[test]
    fn parser_test(){
        let paths = "./";
        let parser = ReleasePraser::new(paths);
        let dic  = praser::prase();
        println!("{}", dic);
    }

}