# Publisher (pbl)

Publisher is a simple command line application for rendering templates from structured data
and a template.

## Usage

```
A simple command line application for rendering templates from structured data and a template.

USAGE:
    pbl [OPTIONS]

OPTIONS:
    -d, --data <DATA>            Path to the data file [env: PBL_DATA=] [default: data.json]
    -h, --help                   Print help information
    -o, --output <OUTPUT>        Output file [env: PBL_OUTPUT=] [default: ]
    -t, --template <TEMPLATE>    Path to the template file [env: PBL_TEMPLATE=] [default:
                                 template.mustache]
    -v, --verbose                Verbose mode for debugging
    -V, --version                Print version information
```

## Examples

Given a data file, `data.json`:
```json
{
  "title": "My Blog",
  "body": "blah"
}
```

and a template, `template.mustache`:
```mustache
<html lang="en">
  <head>
    <title>{{data.title}}</title>
  </head>
  <body>
    <p>Author: {{env.USER}}</p>
    {{data.body}}
  </body>
</html>
```

the following command will render the template:
```shell
pbl render --data data.json --template template.mustache
```

By default, the result is written to `stdout` and will look like:
```html
<html lang="en">
<head>
  <title>My Blog</title>
</head>
<body>
blah
</body>
</html>
```


## Why was pbl created?

The first use case for `pbl` was to create a templated `CONTRIBUTING.md` 
document for a project given a data file describing the project.

## Roadmap

* Add support for transformations.
* Add support for [json-e](https://json-e.js.org/).
* Ability to specify output path(s).
* Add support for multiple template engines.
* Add support for multiple data formats.
* Given a template, parse the data it requires and return an empty 
  serialized data file or prompt for values. 
