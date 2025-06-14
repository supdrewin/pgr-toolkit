use crate::prelude::*;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, value_name = "INDEX")]
    pub mirror: Option<usize>,

    #[arg(short, long, value_name = "NUMBER")]
    pub threads: Option<usize>,

    #[arg(short, long, value_name = "DIR")]
    pub path: Option<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
