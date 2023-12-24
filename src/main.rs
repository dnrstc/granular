use clap::Parser;
use color_eyre::eyre::{Context, Result};

#[derive(Parser)]
struct Cli {
    html_path: std::path::PathBuf,
    css_path: std::path::PathBuf,
}

fn read_file(path: &str) -> color_eyre::Result<String> {
    let contents = std::fs::read_to_string(path).wrap_err_with(|| format!("reading {path}"))?;
    Ok(contents)
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

    println!("{html}");
    println!("{css}");

    Ok(())
}
