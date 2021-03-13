#[macro_use]
extern crate clap;
extern crate mustache;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use mustache::{Data, MapBuilder, Template};
use serde_json::{Value, Map};
use std::io;

mod model;

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches: ArgMatches = App::new("publisher")
        .version("0.0.1")
        .author("Shane Witbeck")
        .about("Publisher")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .arg(
            Arg::with_name("debug")
                .help("Debug.")
                .short("d")
        )
        .subcommand(
            SubCommand::with_name("render")
                .about("Render an artifact")
                .arg(
                    Arg::with_name("data")
                        .help("Data path.")
                        .long("data")
                        .takes_value(true)
                        .env("PB_DATA")
                        .default_value("pb-data.json")
                        .required(true),
                )
                .arg(
                    Arg::with_name("file")
                        .help("Save rendered content to a file instead of stdout.")
                        .short("f")
                        .long("file")
                        .takes_value(false)
                        .default_value("pb-output")
                )
                .arg(
                    Arg::with_name("template")
                        .help("Template to use.")
                        .short("t")
                        .long("template")
                        .takes_value(true)
                        .default_value("pb-template.mustache")
                        .required(true),
                )
        )
        .get_matches();

    let debug = arg_matches.is_present("debug");

    match arg_matches.subcommand() {
        ("render", Some(sub_m)) => {
            if debug {
                eprintln!("{:#?}", sub_m);
            }
            let render_config = RenderConfig::from_args(debug, sub_m);
            if debug {
                eprintln!("{:#?}", render_config);
            }
            render_handler(render_config).expect("Error rendering");
        }
        _ => {}
    }

    Ok(())
}


fn render_handler(render_config: RenderConfig) -> Result<(), Box<dyn Error>> {
    let path_buf: PathBuf = [".", &render_config.data].iter().collect();
    let file: File = File::open(&path_buf)
        .unwrap_or_else(|_| panic!("Error opening file: {}", path_buf.to_str().unwrap()));
    let reader: BufReader<File> = BufReader::new(file);

    let data: Value = serde_json::de::from_reader(reader).expect("Error deserializing data");

    // Define the data for the template
    let data: Data = MapBuilder::new()
        .insert("data", &data).expect("Error inserting data")
        .build();

    // Define the template
    let template: String = render_config.template;
    let template_path: PathBuf = [".", &template].iter().collect();
    let template: Template = mustache::compile_path(template_path).expect("Error reading or compiling template.");

    // Render
    match render_config.file {
        Some(file) => {

            // Create the output file
            let output_path: PathBuf = PathBuf::from(file);
            let output_file: File = File::create(&output_path).expect("Error creating output file.");

            // Write the rendered data to the file specified
            let mut output_buf: BufWriter<File> = BufWriter::new(output_file);
            template.render_data(&mut output_buf, &data).expect("Error rendering output.");
        }
        None => {
            // Write the rendered data to stdout
            template.render_data(&mut io::stdout(), &data).expect("Error rendering output.");
        }
    }
    Ok(())
}

#[derive(Debug)]
struct RenderConfig {
    data: String,
    template: String,
    file: Option<String>,
}

impl RenderConfig {
    pub fn from_args(debug: bool, arg_matches: &ArgMatches) -> RenderConfig {
        let data = arg_matches.value_of("data").expect("Missing data");
        let template = arg_matches.value_of("template").expect("Missing template");
        let file = arg_matches.value_of("file");
        let pfile = match file {
            Some(f) => Option::from(f.to_string()),
            None => None
        };
        RenderConfig::new(data.to_string(), template.to_string(), pfile)
    }

    pub fn new(data: String,
               template: String,
               file: Option<String>) -> RenderConfig {
        RenderConfig {
            data,
            template,
            file,
        }
    }
}