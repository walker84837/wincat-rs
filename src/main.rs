use anyhow::{bail, Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
    os::windows::io::AsRawHandle,
    path::PathBuf,
};
use winapi::um::{errhandlingapi::GetLastError, fileapi, handleapi, processenv, winbase};
use log::info;

/// wincat: A Windows port of the `cat` coreutils program.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[clap(name = "FILEs")]
    input_files: Vec<PathBuf>,

    #[clap(short, long, help = "verbose mode")]
    verbose: bool,
}

fn main() -> Result<()> {
    const HANDLE_FLAG_INHERIT: u32 = 0x00000001;
    let args = Args::parse();

    if args.verbose {
        simple_logger::init().unwrap();
    }

    for file_path in &args.input_files {
        let file = File::open(&file_path)
            .with_context(|| format!("Error opening file: {}", file_path.display()))?;

        info!("Successfully opened file '{}'", &file_path.display());

        let raw_handle = file.as_raw_handle();
        if unsafe {
            handleapi::SetHandleInformation(
                raw_handle as *mut _,
                HANDLE_FLAG_INHERIT,
                HANDLE_FLAG_INHERIT,
            )
        } == 0
        {
            bail!("Error setting file handle to be inheritable: {}", unsafe {
                GetLastError()
            });
        }

        let mut buffer = vec![0u8; 4096];
        let mut reader = BufReader::new(file);

        let stdout_handle = unsafe { processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE) };

        info!("Opened handle to stdout");

        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(err) => bail!("Error reading file: {}", err),
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
                bail!("Error writing to stdout: {}", unsafe { GetLastError() });
            }
        }
    }

    Ok(())
}
