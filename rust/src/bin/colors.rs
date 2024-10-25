use clap::Parser;
use std::path::Path;
use lib::colors::*;
fn main() {
    let args = Args::parse();

    // from https://github.com/meodai/color-names
    let translated_file: &Path = Path::new(&args.i18n_path);
    assert!(translated_file.exists(), "{} not exists", args.i18n_path);

    if args.update {
        let original_file = Path::new(&args.source);
        assert!(original_file.exists(), "{} not exists", args.source);
        _= update(original_file, translated_file);
        return;
    }

    use Supported::*;
    let target = vec![
        Html, Js, JsonSeparated
    ];
    run(translated_file, target);
}
