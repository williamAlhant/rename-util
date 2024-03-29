use syscalls::{syscall, Sysno};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::ffi::CString;
use nix::libc::AT_FDCWD;
use nix::fcntl::RenameFlags;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Exchange 2 paths atomically")]
struct Cli {
    #[arg(help = "A path")]
    old: PathBuf,
    
    #[arg(help = "A path")]
    new: PathBuf,
}

fn main() -> Result<(), syscalls::Errno> {
    let cli = Cli::parse();
    let old = CString::new(cli.old.as_os_str().as_bytes()).unwrap();
    let new = CString::new(cli.new.as_os_str().as_bytes()).unwrap();
    let res = unsafe {
        syscall!(
            Sysno::renameat2,
            AT_FDCWD,
            old.as_ptr(),
            AT_FDCWD,
            new.as_ptr(),
            RenameFlags::RENAME_EXCHANGE.bits()
        )
    };
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
    res.map(|_| ())
}
