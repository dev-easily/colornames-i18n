use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use csv::StringRecord;

fn main() -> anyhow::Result<(), Error> {
    let emojis = fs::read_to_string(Path::new("sources/data-by-group.json"));
    let data = serde_json::from_str::<Vec<EmojiGroup>>(emojis.unwrap().as_str()).unwrap();

    // read i18n csv
    let mut i18n_data: Vec<I18nEmojiGroup> = vec![];
    let mut headers: StringRecord = Default::default();
    read_i18n_emojis_from_csv(&mut headers, &mut i18n_data);

    // update i18n csv
    update_i18n_csv(&data, &i18n_data, &headers).expect("TODO: panic message");

    // output
    let mut locales: Vec<String> = vec![];
    headers.iter().skip(3).for_each(|x| locales.push(String::from(x)));
    minimize(&locales, &i18n_data)?
}

fn update_i18n_csv(origin: &Vec<EmojiGroup>, i18n: &Vec<I18nEmojiGroup>, headers: &StringRecord) -> anyhow::Result<(), Error> {
    let target_name = Path::new("target").join("emoji.i18n.updated.csv");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_name)?;
    file.set_len(0).expect("truncate error");

    let mut writer = csv::Writer::from_path(Path::new("target/emoji.i18n.updated.csv")).unwrap();

    // parent,slug,emoji,en,zh-hans...
    let mut header: Vec<String> = vec![];
    headers.iter().for_each(|x| header.push(String::from(x)));

    writeln!(file, "{}", header.join(","))?;

    let mut group_index = 0;
    for origin_group in origin {
        //0,smileys_emotion,,Smileys & Emotion,笑脸和表情
        let mut line: Vec<String> = vec![group_index.to_string(), origin_group.slug.clone(), String::from(""), origin_group.name.clone()];
        //let mut record: StringRecord = StringRecord::from(line);
        let translated = i18n.iter().find(|&x| x.slug == origin_group.slug);
        //如果已经翻译
        if let Some(t) = translated {
            let names = t.names.clone();
            for i in 4..header.len() { // start from zh-hans
                let locale = header.get(i).unwrap();
                if let Some(&ref n) = names.get(locale) {
                    //record.push_field(n);
                    line.push(n.to_string());
                } else {
                    //record.push_field("");
                    line.push(String::from(""))
                }
            }
        }
        writer.write_record(line);
        for emoji in &origin_group.emojis {
            // 0,grinning_face,😀,grinning face,咧嘴笑
            let mut line: Vec<String> = vec![group_index.to_string(), emoji.slug.clone(), emoji.emoji.clone(), emoji.name.clone()];
            //let mut record: StringRecord = StringRecord::from(line);

            let translated = i18n.iter().find(|&x| x.slug == origin_group.slug);
            if let Some(&ref t) = translated {
                let emojis = &t.emojis;
                let translated_emoji = emojis.iter().find(|&x| x.slug == emoji.slug);
                if let Some(&ref e) = translated_emoji {
                    let names = t.names.clone();
                    for i in 4..header.len() { // start from zh-hans
                        let locale = header.get(i).unwrap();
                        if let Some(&ref n) = names.get(locale) {
                            //record.push_field(n);
                            line.push(n.to_string());

                        } else {
                            //record.push_field("");
                            line.push(String::from(""))

                        }
                    }
                }
            }

            //writeln!(file, "{}", line.join(","));
            writer.write_record(line);
        }
        group_index += 1;
    }
    Ok(())
}

fn read_i18n_emojis_from_csv(headers: &mut StringRecord, i18n_data: &mut Vec<I18nEmojiGroup>) {
    let mut reader = csv::Reader::from_path(Path::new("sources/emoji.i18n.csv")).unwrap();
    let heads = reader.headers().unwrap().clone();
    *headers = heads.clone();
    reader.records()
        .for_each(|line| {
            let line = line.unwrap();
            // parent,slug,emoji,en,zh-hans
            let parent = line.get(0).unwrap();
            let slug =  line.get(1).unwrap();
            let emoji =  line.get(2).unwrap();

            let mut names = HashMap::with_capacity(line.len() - 3);
            for i in 3..line.len() {
                if line.get(i).unwrap().trim().len() > 0 {
                    names.insert(heads[i].to_string(), String::from(line.get(i).unwrap().trim()));
                }
            }

            // emoji group
            if emoji.trim() == "" {
                i18n_data.push(I18nEmojiGroup {
                    slug: String::from(slug),
                    emojis: vec![],
                    names,
                })
            } else {
                // emoji
                let emoji = I18nEmoji {
                    emoji: String::from(emoji),
                    slug: String::from(slug),
                    names,
                };
                let x: &mut I18nEmojiGroup = i18n_data.get_mut(parent.parse::<usize>().unwrap()).unwrap();
                x.emojis.push(emoji);
            }
        });
}

fn generate_i18n(data: &Vec<EmojiGroup>) -> Result<(), Error> {
    let target_name = Path::new("target").join("emoji.i18n.csv");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_name)?;
    file.set_len(0).expect("truncate error");
    writeln!(file, "{}", "parent,slug,emoji,en,zh-hans")?;
    let mut group_index = 0;
    for group in data.iter() {
        writeln!(file, "{},\"{}\",\"{}\",\"{}\",\"{}\"", group_index, group.slug, "", group.name, "")?;
        for emoji in &group.emojis {
            writeln!(file, "{},\"{}\",\"{}\",\"{}\",\"{}\"", group_index, emoji.slug, emoji.emoji, emoji.name, "")?
        }
        group_index += 1;
    }
    Ok(())
}

fn minimize(locales: &Vec<String>, data: &Vec<I18nEmojiGroup>) -> Result<Result<(), Error>, Error> {
    for locale in locales {
        let data: Vec<EmojiGroup> = data.iter().map(|x| {
            x.to(locale)
        }).collect();

        let mini = serde_json::to_string(&data).unwrap();
        let target_name = Path::new("target").join(format!("emojis.i18n.{}.json", locale));
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(target_name)?;
        file.set_len(0).expect("truncate error");
        write!(file, "{}", mini)?;
    }
    Ok(Ok(()))
}

#[derive(Serialize, Deserialize, Debug)]
struct EmojiGroup {
    name: String,
    slug: String,
    emojis: Vec<Emoji>
}

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    emoji: String,
    #[serde(skip_serializing)]
    skin_tone_support: bool,
    name: String,
    slug: String,
    #[serde(skip_serializing)]
    unicode_version: String,
    #[serde(skip_serializing)]
    emoji_version: String,
    /// 在输出到非英语语言的单个json时，如果希望搜索英文也能出现字符，需要在这个json中保留英文名称
    #[serde(default)]
    en_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct I18nEmoji {
    emoji: String,
    slug: String,
    names: HashMap<String, String>
}

impl I18nEmoji {
    fn to(&self, locale: &String) -> Emoji {
        let mut r = Emoji {
            slug: self.slug.clone(),
            unicode_version: "".to_string(),
            name: "".to_string(),
            emoji: self.emoji.clone(),
            skin_tone_support: false,
            emoji_version: "".to_string(),
            en_name: "".to_string(),
        };
        let en = self.names.get(&String::from("en")).unwrap();
        if let Some(n) = self.names.get(locale) {
            r.name = n.to_string();
        } else {
            r.name = en.clone();
        }
        if r.name != *en {
            r.en_name = en.clone();
        }
        r
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct I18nEmojiGroup {
    slug: String,
    emojis: Vec<I18nEmoji>,
    names: HashMap<String, String>
}


impl I18nEmojiGroup {
    fn to(&self, locale: &String) -> EmojiGroup {
        let emojis = self.emojis.iter().map(|x| x.to(locale)).collect();
        if let Some(n) = self.names.get(locale) {
            EmojiGroup {
                slug: self.slug.clone(),
                name: n.clone(),
                emojis,
            }
        } else {
            let en = self.names.get(&String::from("en")).unwrap();

            EmojiGroup {
                slug: self.slug.clone(),
                name: en.clone(),
                emojis,
            }
        }
    }
}
