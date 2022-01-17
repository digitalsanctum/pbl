extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::{AppSettings, Parser};
use serde_json::Value;

fn main() {
    let render_config: RenderConfig = RenderConfig::parse();
    render_arbitrary_mustache(render_config).expect("Error rendering");
}

fn read_value_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Project`.
    let value: Value = serde_json::from_reader(reader)?;

    Ok(value)
}

fn render_arbitrary_mustache(render_config: RenderConfig) -> Result<(), Box<dyn Error>> {
    // Read project data from file
    let data_path: PathBuf = PathBuf::from(&render_config.data);
    let val: Value = read_value_from_file(data_path).expect("Error reading data");

    // Read template from file
    let template_path: PathBuf = PathBuf::from(&render_config.template);
    let template_content: String =
        std::fs::read_to_string(template_path).expect("Error reading template");
    let template = mustache::compile_str(&template_content).expect("Error compiling template");

    template.render(&mut io::stdout(), &val).expect("Error rendering template");

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version, author, about, setting = AppSettings::ArgRequiredElseHelp)]
struct RenderConfig {
    /// Path to the data file
    #[clap(short, long)]
    data: String,
    /// Path to the template file
    #[clap(short, long)]
    template: String,
}
