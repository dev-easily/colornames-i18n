use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::colors::types::*;

pub fn read_csv_headers_from_i18n_csv(source: &Path) -> String {
    let source = File::open(source).expect("file not readable");
    let mut reader = BufReader::new(source);
    let mut head: String = String::from("");
    reader.read_line(&mut head).expect("failed to read header");
    head
}
pub fn read_color_names_from_i18n_csv(source: &Path) -> Vec<I18nColor> {
    let source = File::open(source).expect("file not readable");
    let mut reader = BufReader::new(source);
    let mut head: String = String::from("");
    reader.read_line(&mut head).expect("failed to read header");
    let heads = head.trim().split(",").map(|x| x.trim()).collect::<Vec<&str>>();

    reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let line_split = line.split(",").collect::<Vec<&str>>();
            let hex = line_split[1];
            let en_name = line_split[0];
            let mut names = HashMap::with_capacity(line_split.len() - 1);
            names.insert(String::from("en"), vec![String::from(en_name)]);
            for i in 2..line_split.len() {
                if line_split[i].trim().len() > 0 {
                    let back_ups = line_split[i].trim().split("/");
                    names.insert(heads[i].to_string(), back_ups.filter(|x| x.len() > 0).map(|x| String::from(x)).collect());
                }
            }
            I18nColor::new(String::from(en_name), String::from(hex)).with_names(names)
        }).collect()
}

pub fn init_target_file(source: &Path, target_extension: &str) -> anyhow::Result<File> {
    let base_name = source.file_name().unwrap();
    let target_name = Path::new("target").join(base_name).with_extension(target_extension);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_name)?;
    file.set_len(0).expect("truncate error");
    Ok(file)
}

pub fn read_color_names_from_origin_csv(source: &Path) -> Vec<OriginColor> {
    let source = File::open(source).expect("file not readable");
    let reader = BufReader::new(source);
    reader.lines()
        .skip(1)
        .map(|x| {
            let x = x.unwrap();
            let split = x.split(",").collect::<Vec<&str>>();
            (String::from(split[0]), String::from(split[1]))
        })
        .map(|(name, hex)| OriginColor {
            name,
            hex
        }).collect()
}

#[test]
fn test_i18n_read() {
    let colors = read_color_names_from_i18n_csv(Path::new("../sources/colornames.i18n.csv"));
    println!("colors.count: {}", colors.len());
    //println!("{:?}", colors);
    assert_eq!("时速100迈", colors[0].names["zh-hans"][0]);
    assert_eq!("24胡萝卜", colors[2].names["zh-hans"][0]);
    assert_eq!(2, colors[2].names["zh-hans"].len());
    assert!(colors[100].names.get("zh-hans").is_none());
}