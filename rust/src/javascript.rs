use std::io::Write;
use std::path::Path;

use crate::file::*;
pub fn generate(path: &Path) -> anyhow::Result<()> {
    let mut file = init_target_file(path, "js")?;
    let header = read_csv_headers_from_i18n_csv(path);
    let heads = header.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

    writeln!(file, r#"export const headers = "#).expect("TODO: panic message");
    writeln!(file, "{}", serde_json::to_string(&heads)?);

    writeln!(file, r#"export const data = "#).expect("TODO: panic message");

    let colors = read_color_names_from_i18n_csv(path);
    let serialized = serde_json::to_string(&colors).unwrap();
    writeln!(file, "{}", serialized).expect("TODO: panic message");
    Ok(())
}