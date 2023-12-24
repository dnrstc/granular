use color_eyre::eyre::{Context, Result};

fn read_file(path: &str) -> color_eyre::Result<String> {
    let contents = std::fs::read_to_string(path).wrap_err("reading {path}")?;
    Ok(contents)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let html = read_file("index.html")?;
    let css = read_file("style.css")?;

    println!("{html}");
    println!("{css}");

    Ok(())
}
