use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{self, Read},
    os::windows::io::AsRawHandle,
    path::PathBuf,
};
use structopt::StructOpt;
use winapi::um::{fileapi, handleapi, winbase};

const HANDLE_FLAG_INHERIT: u32 = 0x00000001;

/// A Windows port of the `cat` coreutils program.
#[derive(StructOpt)]
#[structopt(name = "cat-win")]
struct Args {
    #[structopt(parse(from_os_str))]
    input_files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let arguments_list: Vec<String> = std::env::args().collect();

    // If no arguments are provided
    if arguments_list.len() == 1 {
        Args::clap().print_help()?;
        println!();
        anyhow::bail!("Please provide at least one argument.");
    }

    for file_path in &args.input_files {
        let file = File::open(&file_path)
            .with_context(|| format!("Error opening file: {}", file_path.display()))?;

        let raw_handle = file.as_raw_handle();
        if unsafe {
            handleapi::SetHandleInformation(
                raw_handle as *mut _,
                HANDLE_FLAG_INHERIT,
                HANDLE_FLAG_INHERIT,
            )
        } == 0
        {
            anyhow::bail!("Error setting file handle to be inheritable");
        }

        let mut buffer = vec![0u8; 4096];
        let mut reader = io::BufReader::new(file);

        let stdout_handle =
            unsafe { winapi::um::processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE) };

        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(err) => anyhow::bail!("Error reading file: {}", err),
            };

            let mut written = 0;
            if unsafe {
                fileapi::WriteFile(
                    stdout_handle,
                    buffer.as_ptr() as *const _,
                    bytes_read as u32,
                    &mut written,
                    std::ptr::null_mut(),
                )
            } == 0
            {
                anyhow::bail!("Error writing to stdout");
            }
        }
    }

    Ok(())
}
