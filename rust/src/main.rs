use clap::Parser;
use lib::Args;
use std::path::Path;

fn main() {
    let args = Args::parse();

    let translated_file: &Path = Path::new(&args.i18n_path);
    assert!(translated_file.exists(), "{} not exists", args.i18n_path);

    if args.update {
        let original_file = Path::new(&args.source);
        assert!(original_file.exists(), "{} not exists", args.source);
        _= lib::update(original_file, translated_file);
        return;
    }

    use lib::Supported::*;
    let target = vec![
        Html, Js
    ];
    lib::run(translated_file, target);
}
