const std = @import("std");

pub fn main() !void {
	const stdout = std.io.getStdOut().writer();
	const argc = std.os.argv.len;

	// Use arg as input filename otherwise assume "input".
	const input_filename = if (argc == 2) std.os.argv[1] else @constCast("input");

	try stdout.print("Opening input file {s}..\n", .{input_filename});

	// Open file
	const input_file = std.fs.cwd().openFileZ(input_filename, std.fs.File.OpenFlags {}) catch {
		std.debug.print("Could not open file!\n", .{});
		return;
	};
	defer input_file.close();
}
