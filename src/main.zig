const std = @import("std");
const args = @import("args");
const win = std.os.windows;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const options = try args.parseForCurrentProcess(struct {
        // Command line options
        verbose: bool = false,

        // Shorthand options
        pub const shorthands = .{
            .v = "verbose",
        };

        pub const meta = .{
            .about = "wincat: A Windows port of the `cat` coreutils program.",
        };
    }, allocator, .print);
    defer options.deinit();

    // Setup logging if verbose is enabled
    const verbose = options.options.verbose;
    if (verbose) {
        std.log.info("Verbose mode enabled", .{});
    }

    // Process each input file
    for (options.positionals) |file_path| {
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
            std.debug.print("Error opening file '{s}': {s}\n", .{ file_path, @errorName(err) });
            return err;
        };
        defer file.close();

        if (verbose) {
            std.log.info("Successfully opened file '{s}'", .{file_path});
        }

        // Make file handle inheritable
        const HANDLE_FLAG_INHERIT: u32 = 0x00000001;
        const handle = file.handle;

        if (win.kernel32.SetHandleInformation(handle, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT) == 0) {
            const err = win.kernel32.GetLastError();
            std.debug.print("Error setting file handle to be inheritable: {d}\n", .{err});
            return error.SetHandleInformationFailed;
        }

        // Get stdout handle
        const stdout_handle = win.kernel32.GetStdHandle(win.STD_OUTPUT_HANDLE);
        if (verbose) {
            std.log.info("Opened handle to stdout", .{});
        }

        // Read and write file contents
        var buffer: [4096]u8 = undefined;

        while (true) {
            const bytes_read = file.read(&buffer) catch |err| {
                std.debug.print("Error reading file: {s}\n", .{@errorName(err)});
                return err;
            };

            if (bytes_read == 0) {
                break;
            }

            var written: u32 = 0;
            const nullPointer: *anyopaque = null;

            if (win.kernel32.WriteFile(stdout_handle, &buffer, @intCast(bytes_read), &written, nullPointer) == 0) {
                const err = win.kernel32.GetLastError();
                std.debug.print("Error writing to stdout: {d}\n", .{err});
                return error.WriteFileFailed;
            }
        }
    }
}
