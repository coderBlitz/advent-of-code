const std = @import("std");

/// Split the given buffer on `delimiter` and end-of-buffer, appending slices to
///  the given ArrayList for each field.
fn split(arr: *std.ArrayList([]u8), buf: []u8, delimiter: u8) !usize {
    var start: usize = 0;
    var end: usize = 0;
    var fields: usize = 1;

    // Iterate the characters and append a new slice when a delimiter is found.
    // At the end of an iteration where `c == delimiter`, `start == end`.
    for (buf) |c| {
        if (c == delimiter) {
            try arr.append(buf[start..end]);
            fields += 1;
            start = end + 1;
        }
        end += 1;
    }

    // Always append remaining slice, even if empty.
    try arr.append(buf[start..end]);

    return fields;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const argc = std.os.argv.len;

    // Use arg as input filename otherwise assume "input".
    const input_filename = if (argc == 2) std.os.argv[1] else @constCast("input");

    try stdout.print("Opening input file {s}..\n", .{input_filename});

    // Open file
    const input_file = std.fs.cwd().openFileZ(input_filename, std.fs.File.OpenFlags{}) catch {
        std.debug.print("Could not open file!\n", .{});
        return;
    };
    defer input_file.close();

    // Iterate lines.
    const reader = input_file.reader();

    var alloc_buf = std.mem.zeroes([2048]u8);
    var buf_alloc = std.heap.FixedBufferAllocator.init(&alloc_buf);
    const alloc = buf_alloc.allocator();

    var buf: [4096]u8 = undefined;
    var i: usize = 0;
    var fields = std.ArrayList([]u8).init(alloc);
    defer fields.deinit();

    var safe: usize = 0;
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;

        // Test split on commas (simple CSV)
        _ = try split(&fields, line, ' ');

        var prod: isize = 0;
        var diff: isize = 0;
        var row_safe: bool = true;
        var j: usize = 1;
        while (j < fields.items.len) : (j += 1) {
            const b = try std.fmt.parseInt(isize, fields.items[j], 10);
            const a = try std.fmt.parseInt(isize, fields.items[j - 1], 10);
            diff = b - a;
            prod = diff * prod;
            if (prod < 0 or @abs(diff) < 1 or @abs(diff) > 3) {
                row_safe = false;
                break;
            }

            if (diff >= 0) {
                prod = 1;
            } else {
                prod = -1;
            }
        }

        if (row_safe) {
            safe += 1;
        }

        fields.clearRetainingCapacity();
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    try stdout.print("{d} reports are safe.\n", .{safe});
}
