use std::vec;

use clap::Parser;
use color_eyre::eyre::{Context, Result};

#[derive(Parser)]
struct Cli {
    html_path: std::path::PathBuf,
    css_path: std::path::PathBuf,
}

#[derive(Debug)]
struct CssObject {
    selector: String,
    rules: Vec<String>,
}

fn read_file(path: &str) -> color_eyre::Result<String> {
    let contents = std::fs::read_to_string(path).wrap_err_with(|| format!("reading {path}"))?;
    Ok(contents)
}

fn parse_classes(html_contents: &str) -> Result<()> {
    let mut classes: Vec<&str> = vec![];

    for line in html_contents.lines() {
        let class = line.split_once("class=");
        match class {
            Some((_, class_name)) => {
                let new_class_name = class_name.split("\"").collect::<Vec<_>>()[1];
                classes.push(new_class_name);
            }
            None => (),
        }
    }

    println!("{classes:?}");

    Ok(())
}

fn parse_css(css_contents: &str) -> Result<()> {
    let mut styles = Vec::new();
    let mut selector = String::new();
    let mut rules = Vec::new();

    for line in css_contents.lines() {
        if line.starts_with('.') {
            selector = line.split('{').next().unwrap().trim().to_string();
        } else if !line.contains('}') {
            if !line.trim().is_empty() {
                rules.push(line.trim().to_string());
            }
        } else if line.contains('}') {
            let css_object = CssObject {
                selector: selector.clone(),
                rules: rules.clone(),
            };

            selector.clear();
            rules.clear();

            styles.push(css_object);
        }
    }

    println!("{:?}", styles);

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let Cli {
        css_path,
        html_path,
    } = Cli::parse();

    let html_path = html_path.to_str().unwrap();
    let css_path = css_path.to_str().unwrap();

    let html = read_file(html_path)?;
    let css = read_file(css_path)?;

    parse_classes(&html);
    parse_css(&css);

    Ok(())
}
