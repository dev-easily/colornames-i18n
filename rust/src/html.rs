use colors_transform::{Color, Rgb};
use std::path::Path;
use std::io::Write;

use crate::file::*;

pub fn generate(path: &Path) -> anyhow::Result<()> {
    let mut file = init_target_file(path, "html")?;

    writeln!(file, r#"
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Document</title>
    <style>
        .content {{
            display: flex;
            justify-content: center;
        }}
        table tr td:last-child {{
            width: 300px;
        }}
    </style>
</head>
<body>

<script>
    var locale
    function query(element) {{
        window.open("https://www.deepl.com/en/translator#en/zh-hans/" + encodeURI(element.innerText))
    }}
</script>

<div class="content">
<table>
    <thead>
    <tr>
        <td>Color Name</td>
        <td>HEX</td>
        <td>RGB</td>
        <td>HSL</td>
        <td>Color</td>
    </tr>
    </thead>
    <tbody>
    "#).expect("error while writing");

    read_color_names_from_origin_csv(path)
        .iter()
        .for_each(|color| {
            let name = color.name.clone();
            let hex = color.hex.clone();
            let rgb = Rgb::from_hex_str(&hex[1..]).expect("failed to convert to rgb value");
            let hsl = rgb.to_hsl();
            let rgb_value = rgb.to_css_string();
            let hsl_value = hsl.to_css_string();
            if let Err(e) = writeln!(file, r#"<tr><td onClick="query(this)">{name}</td><td>{hex}</td><td>{rgb_value}</td><td>{hsl_value}</td><td bgcolor="{hex}"></td></tr>"#) {
                eprintln!("error while writing {}", e);
            }
        })
    ;

    writeln!(file, r#"
    </tbody>
</table>
<div>
</body>
</html>
    "#).expect("error while writing");

    Ok(())
}