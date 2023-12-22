use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::PathBuf};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct Rgb(u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Color {
    Name(String),
    Rgb(Rgb),
}

#[derive(Debug, Deserialize, Serialize)]
struct Theme {
    palette: HashMap<String, Rgb>,
    editor: HashMap<String, Color>,
    terminal: HashMap<String, Color>,
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let handlebars = Handlebars::new();
    for theme_id in &["cosmic_dark", "cosmic_light"] {
        let theme_path = format!("themes/{}.toml", theme_id);
        println!("cargo:rerun-if-changed={}", theme_path);
        let theme_toml = fs::read_to_string(&theme_path).expect("failed to read theme file");
        let theme: Theme = toml::from_str(&theme_toml).expect("failed to parse theme file");

        // Generate text editor theme
        {
            let template_path = format!("templates/{}.tmTheme", theme_id);
            println!("cargo:rerun-if-changed={}", template_path);
            let template =
                fs::read_to_string(&template_path).expect("failed to read editor template");

            let mut data = HashMap::new();
            for (name, value) in theme.palette.iter() {
                data.insert(name.clone(), format!("#{:06X}", value.0));
            }
            for (name, color) in theme.editor.iter() {
                let value = match color {
                    Color::Name(palette_name) => theme.palette[palette_name],
                    Color::Rgb(value) => *value,
                };
                data.insert(name.clone(), format!("#{:06X}", value.0));
            }

            let rendered = handlebars
                .render_template(&template, &data)
                .expect("failed to render editor template");
            fs::write(out_dir.join(format!("{}.tmTheme", theme_id)), rendered)
                .expect("failed to write generated editor theme");
        }

        //TODO: generate terminal theme
    }
}
