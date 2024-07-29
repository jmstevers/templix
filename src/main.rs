use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A simple templating tool.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new project using a template.
    Init {
        /// Name of the template to use.
        template_name: String,

        /// The name of the project.
        project_name: String,

        /// The path where the project should be initialized.
        #[arg(default_value = ".")]
        project_path: Option<String>,

        /// The path to the templates directory.
        #[arg(short, long)]
        templates_path: Option<String>,
    },
    /// List all available templates.
    List {
        /// The path to the templates directory.
        #[arg(short, long)]
        templates_path: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init {
            template_name,
            project_name,
            project_path,
            templates_path,
        } => init(
            templates_path,
            &template_name,
            &project_name,
            project_path.unwrap_or(".".into()).into(),
        )?,
        Commands::List { templates_path } => list(templates_path)?,
    }

    Ok(())
}

fn init(
    templates_path: Option<String>,
    template_name: &str,
    name: &str,
    path: PathBuf,
) -> anyhow::Result<()> {
    let templates_path = match templates_path {
        Some(path) => Path::new(&path).join(template_name),
        None => {
            let templates_env = std::env::var_os("templates")
                .ok_or(anyhow::anyhow!("No templates directory set"))?;
            Path::new(&templates_env).join(template_name)
        }
    };

    let path_str = path.to_str().unwrap();
    let walk_dir = WalkDir::new(&templates_path);
    let project_path = path.join(name);

    for entry in walk_dir.into_iter() {
        let entry = entry?;
        let copy_path = entry.path();
        let write_path = project_path.join(copy_path.strip_prefix(&templates_path).unwrap());

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(write_path)?;
        } else if entry.file_type().is_file() {
            let content = std::fs::read_to_string(copy_path)?
                .replace("templix{name}", name)
                .replace("templix{path}", path_str);

            std::fs::write(write_path, content)?;
        }
    }

    Ok(())
}

fn list(templates_path: Option<String>) -> anyhow::Result<()> {
    let templates_path: PathBuf = match templates_path {
        Some(path) => path.into(),
        None => {
            let templates_env = std::env::var_os("templates")
                .ok_or(anyhow::anyhow!("No templates directory set"))?;
            Path::new(&templates_env).to_path_buf()
        }
    };

    for entry in std::fs::read_dir(&templates_path)? {
        let entry = entry?;
        let copy_path = entry.path();
        let write_path = copy_path.strip_prefix(&templates_path).unwrap();

        if entry.file_type()?.is_dir() {
            println!("{}", write_path.display());
        }
    }

    Ok(())
}
