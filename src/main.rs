extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::Parser;
use serde_derive::Serialize;
use serde_json::Value;
use chrono::{Datelike, Local, Weekday};


fn main() {
    let render_config: RenderConfig = RenderConfig::parse();
    if render_config.verbose {
        println!("{:?}", render_config);
    }
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
    if render_config.verbose {
        println!("{:?}", val);
    }

    // Augment data with configuration, environment variables, and date
    let augmented_data = AugmentedData {
        data: val.clone(),
        env: std::env::vars().collect(),
        config: render_config.clone(),
        formatted_date: formatted_date(),
    };

    if render_config.verbose {
        println!("{:?}", augmented_data);
    }

    // Read template from file
    let template_path: PathBuf = PathBuf::from(&render_config.template);
    let template_content: String =
        std::fs::read_to_string(template_path).expect("Error reading template");
    if render_config.verbose {
        println!("{:?}", template_content);
    }
    let template = mustache::compile_str(&template_content).expect("Error compiling template");

    // Render template to stdout
    template.render(&mut io::stdout(), &augmented_data).expect("Error rendering template");

    // if output file is specified, write to file
    if render_config.output != "" {
        let output_path: PathBuf = PathBuf::from(&render_config.output);
        let mut output_file = File::create(output_path).expect("Error creating output file");
        template.render(&mut output_file, &augmented_data).expect("Error rendering template");
    }

    Ok(())
}

fn formatted_date() -> String {
    let now = Local::now();
    let weekday = match now.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    let month = match now.month() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("Invalid month"),
    };
    let formatted_date = format!("{} - {} {}, {}", weekday, month, now.day(), now.year());
    return formatted_date;
}

#[derive(Parser, Debug, Serialize, Clone)]
#[clap(version, author, about)]
struct RenderConfig {
    /// Path to the data file
    #[clap(short, long, default_value = "data.json", env = "PBL_DATA")]
    data: String,
    /// Path to the template file
    #[clap(short, long, default_value = "template.mustache", env = "PBL_TEMPLATE")]
    template: String,
    /// Verbose mode for debugging
    #[clap(short, long)]
    verbose: bool,
    /// Output file
    #[clap(short, long, default_value = "", env = "PBL_OUTPUT")]
    output: String,
}


#[derive(Serialize, Debug)]
struct AugmentedData {
    data: Value,
    env: std::collections::HashMap<String, String>,
    config: RenderConfig,
    formatted_date: String,
}