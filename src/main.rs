use std::{
    io::{self, Read},
    fs::File,
    os::windows::io::AsRawHandle,
    path::PathBuf,
    process::ExitCode,
};
use winapi::um::{
    handleapi::SetHandleInformation,
    winbase::STD_OUTPUT_HANDLE
};
use structopt::StructOpt;

const HANDLE_FLAG_INHERIT: u32 = 0x00000001;

/// A Windows port of the `cat` coreutils program.
#[derive(StructOpt)]
#[structopt(name = "cat-win")]
struct Args {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

fn main() -> ExitCode {
    let args = Args::from_args();

    let file_path = args.input_file;

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let raw_handle = file.as_raw_handle();
    if unsafe { SetHandleInformation(raw_handle as *mut _, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT) } == 0 {
        eprintln!("Error setting file handle to be inheritable");
        return ExitCode::FAILURE;
    }

    let mut buffer = vec![0u8; 4096];
    let mut reader = io::BufReader::new(file);

    let stdout_handle = unsafe { winapi::um::processenv::GetStdHandle(STD_OUTPUT_HANDLE) };

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                return ExitCode::FAILURE;
            }
        };

        let mut written = 0;
        if unsafe {
            winapi::um::fileapi::WriteFile(
                stdout_handle,
                buffer.as_ptr() as *const _,
                bytes_read as u32,
                &mut written,
                std::ptr::null_mut(),
            )
        } == 0
        {
            eprintln!("Error writing to stdout");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
