#![allow(unused)]

use std::collections::HashMap;
use std::fs::OpenOptions;
use clap::Parser;
use colors_transform::{Color, Rgb};
use std::path::Path;
use crate::file::*;
use crate::types::I18nColor;
use std::io::Write;

mod types;
mod file;
mod javascript;
mod html;
mod json_separated;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 是否需要从 colornames.bestof.csv 更新 colornames.i18n.csv
    #[arg(short, long)]
    pub update: bool,

    /// path of the csv file to parse
    #[arg(short, long, default_value = "csv/colornames.bestof.csv")]
    pub source: String,

    /// path of the i18n csv file to parse
    #[arg(short, long, default_value = "i18n/colornames.i18n.csv")]
    pub i18n_path: String,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_color_convert() {
        let hex = "#1234fe";
        let rgb_value = Rgb::from_hex_str(hex[1..].to_string().as_str()).expect("failed to convert to rgb value");
        let hsl = rgb_value.to_hsl();
        println!("{}", rgb_value.to_css_string());
        println!("{}", hsl.to_css_string());
    }
}

pub enum Supported {
    Origin,
    Html,
    Js,
    JsonSeparated,
}

pub fn run(path: &Path, p1: Vec<Supported>) {
    use Supported::*;
    for target in p1 {
        match target {
            Origin => {
                _ = html::generate(path);
            },
            Html => {
                _ = html::generate(path);
            },
            Js => {
                _ = javascript::generate(path);
            },
            JsonSeparated => {
                _ = json_separated::generate(path);
            }
        }
    }
}

/// add colors or change english color names from `colornames.bestof.csv`
pub fn update(origin: &Path, i18n: &Path) -> anyhow::Result<()> {
    let origin_colors = read_color_names_from_origin_csv(origin);
    let mut translated = HashMap::new();

    let binding = read_color_names_from_i18n_csv(i18n);
    binding
        .iter()
        .for_each(|color| {
            let hex = color.hex.clone();
            translated.insert(hex, color);
        });

    let head = read_csv_headers_from_i18n_csv(i18n);
    let heads = head.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

    let mut updated_file = init_target_file(i18n, "updated.csv").expect("failed to init updated.csv, check to see if directory exists");
    writeln!(updated_file, "{}", head.trim());
    for color in origin_colors {
        let hex = color.hex;
        if let Some(i18n) = translated.get(&hex) {
            let en = i18n.names.get("en").unwrap().get(0).unwrap();
            let hex = i18n.hex.clone();
            let mut line = format!("{:?},{:?}", en, hex);
            for i in 2..heads.len() {
                line.push(',');
                if let Some(vec) = i18n.names.get(heads[i]) {
                    line.push_str(vec.join("/").as_str());
                }
            }
            writeln!(updated_file, "{}", line);
        } else {
            // new color
            writeln!(updated_file, "{},{}", color.name, hex);
        }
    }
    println!("done, check target/*.updated.csv and rename it to colornames.i18n.csv before commit.");
    Ok(())
}