use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
    os::windows::io::AsRawHandle,
    path::PathBuf,
};
use winapi::um::{fileapi, handleapi, processenv, winbase};

/// wincat-rs: A Windows port of the `cat` coreutils program.
#[derive(Parser)]
struct Args {
    #[arg(name = "FILEs")]
    input_files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    const HANDLE_FLAG_INHERIT: u32 = 0x00000001;

    let args = Args::parse();

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
            return Err(anyhow!("Error setting file handle to be inheritable"));
        }

        let mut buffer = vec![0u8; 4096];
        let mut reader = BufReader::new(file);

        let stdout_handle = unsafe { processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE) };

        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(err) => return Err(anyhow!("Error reading file: {}", err)),
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
                return Err(anyhow!("Error writing to stdout"));
            }
        }
    }

    Ok(())
}
