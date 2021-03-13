extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use ramhorns::Template;

use crate::model::project::Project;

mod model;

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches: ArgMatches = App::new("publisher")
        .version("0.1.1")
        .author("Shane Witbeck")
        .about("Publisher")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .arg(Arg::with_name("debug").help("Debug").short("d"))
        .subcommand(
            SubCommand::with_name("render")
                .about("Render an artifact given a data file and template.")
                .arg(
                    Arg::with_name("data")
                        .help("A JSON data file")
                        .long("data")
                        .takes_value(true)
                        .env("PBL_DATA")
                        .default_value("pbl-data.json")
                        .required(true),
                )
                .arg(
                    Arg::with_name("template")
                        .help("Mustache template")
                        .short("t")
                        .long("template")
                        .takes_value(true)
                        .env("PBL_TEMPLATE")
                        .default_value("pbl-template.mustache")
                        .required(true),
                ),
        )
        .get_matches();

    let debug = arg_matches.is_present("debug");

    match arg_matches.subcommand() {
        ("render", Some(sub_m)) => {
            if debug {
                eprintln!("{:#?}", sub_m);
            }
            let render_config = RenderConfig::from_args(sub_m);
            if debug {
                eprintln!("{:#?}", render_config);
            }
            render_handler(render_config).expect("Error rendering");
        }
        _ => {}
    }

    Ok(())
}

fn read_project_from_file<P: AsRef<Path>>(path: P) -> Result<Project, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Project`.
    let project: Project = serde_json::from_reader(reader)?;

    Ok(project)
}

fn render_handler(render_config: RenderConfig) -> Result<(), Box<dyn Error>> {
    // Read project data from file
    let path_buf: PathBuf = [".", &render_config.data].iter().collect();
    let project: Project = read_project_from_file(path_buf).expect("Error reading project data");

    // Read template from file
    let template: String = render_config.template;
    let template_path: PathBuf = [".", &template].iter().collect();
    let template_content: String =
        std::fs::read_to_string(template_path).expect("Error reading template file");
    let template: Template = Template::new(template_content).expect("Error instantiating template");

    // Render
    let rendered = template.render(&project);
    println!("{}", rendered);

    Ok(())
}

#[derive(Debug)]
struct RenderConfig {
    data: String,
    template: String,
}

impl RenderConfig {
    pub fn from_args(arg_matches: &ArgMatches) -> RenderConfig {
        let data = arg_matches.value_of("data").expect("Missing data");
        let template = arg_matches.value_of("template").expect("Missing template");
        RenderConfig::new(data.to_string(), template.to_string())
    }

    pub fn new(data: String, template: String) -> RenderConfig {
        RenderConfig { data, template }
    }
}
