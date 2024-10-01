use std::io::Write;
use std::path::Path;

use crate::file::*;
use crate::types::*;

pub fn generate(path: &Path) -> anyhow::Result<()> {
    let header = read_csv_headers_from_i18n_csv(path);
    let heads = header.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

    let mut locales = vec!["en"];
    locales.extend(&heads[2..]);

    for locale in locales {
        let mut file = init_target_file(path, format!("{}.json", locale).as_str())?;
        write!(file, r#"{{"colors":"#).expect("TODO: panic message");
        let colors = read_color_names_from_i18n_csv(path);
        let simple_colors: Vec<OriginColor> = colors.iter().map(|c| c.to_simple(&String::from(locale))).collect();
        let serialized = serde_json::to_string(&simple_colors).unwrap();
        write!(file, "{}", serialized).expect("TODO: panic message");
        writeln!(file, "}}");
    }

    Ok(())
}