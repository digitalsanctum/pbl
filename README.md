# Publisher (pbl)

Publisher is a simple command line application for creating artifacts from data
and a mustache template.

## Usage

```
Render an artifact given a data file and template.

USAGE:
    pbl render --data <data> --template <template>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --data <data>            A JSON data file [env: PBL_DATA=]  [default: pbl-data.json]
    -t, --template <template>    Mustache template [env: PBL_TEMPLATE=]  [default: pbl-template.mustache]
```

## Why was pbl created?

The first use case for `pbl` was to create a templated `CONTRIBUTING.md` 
document for a project given a data file describing the project.

## Roadmap

* Support additional artifacts.
* Given a Mustache template, parse the data it requires and return an empty 
  serialized data file. 
