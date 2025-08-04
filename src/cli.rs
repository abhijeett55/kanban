use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]

pub enum Action {
    Configure {},
    Show {
        #[structopt(short, long)]
        json: bool,
    },

    Add{
        #[structopt()]
        task: String,
    },

    Promote {
        #[structopt()]
        id: u32,
    },

    Regress {
        #[structopt()]
        id: u32,
    },
    Delete {
        #[structopt()]
        id: u32,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "kanban", about = "A command line kanban board written in Rust.")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub config_file: Option<PathBuf>,
}