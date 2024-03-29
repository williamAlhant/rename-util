use nix::fcntl::{renameat2, RenameFlags};
use std::path::PathBuf;
use nix::libc::AT_FDCWD;
use nix::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Exchange 2 paths atomically")]
struct Cli {
    #[arg(help = "A path")]
    old: PathBuf,
    
    #[arg(help = "A path")]
    new: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let res = renameat2(Some(AT_FDCWD), &cli.old, Some(AT_FDCWD), &cli.new, RenameFlags::RENAME_EXCHANGE);
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
    res
}
