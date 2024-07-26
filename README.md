# Templix

A simple templating tool.

## Build From Source

First clone the project, then

### With Cargo

Run `cargo build -r` in the root directory.

### With Nix

Run `nix build -f default.nix` in the root directory.

## Usage

Create an environment variable `TEMPLATES` and set it to the path where you'll store your templates (or specify the path with `--templates-path`).

### Create a Template

Create a new directory with the desired name (Templix uses the directory name during initialization) and add the necessary files to it. Inside each file, you can optionally include the `templix{name}` and `templix{path}` placeholders, which will be replaced by the project name and the root path of the project, respectively, when using Templix.

### Initialize a Template

To initialize a template with Templix, use the following command:

```
templix init <template_name> <path/to/project>
```

For example, to initialize a template named "zig" in the current directory, you can run:

```
templix init zig
```

If you want to specify a custom path for the project, you can provide the `path/to/project` argument. Otherwise, the path will default to the current directory.

### List Out Templates

To list out the templates, use the following command:

```
templix list
```

This will display a list of all the templates available in the `TEMPLATES` directory. You can set the path to the templates by passing the `--templates-path` argument.


## License
[APACHE](https://www.github.com/jmstevers/templix/LICENSE-APACHE) or
[MIT](https://www.github.com/jmstevers/templix/LICENSE-MIT)