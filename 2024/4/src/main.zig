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

    var page_alloc = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const alloc = page_alloc.allocator();

    var buf: [4096]u8 = undefined;
    var i: usize = 0;
    var grid = std.ArrayList(std.ArrayList(u8)).init(alloc);
    defer grid.deinit();
    var sum: usize = 0;
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;
        //try stdout.print("Line {d} length: {d}\n", .{ i + 1, line.len });
        if (line.len == 0) {
            break;
        }

        var row = std.ArrayList(u8).init(alloc);
        try row.appendSlice(line);
        try grid.append(row);
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    i = 0;
    var j: usize = 0;
    while (i < grid.items.len) : (i += 1) {
        const row = grid.items[i];
        j = 0;
        var vbuf = [_]u8{ 0, 0, 0, 0 };
        while (j < row.items.len) : (j += 1) {
            // Search horizontal
            const line = row.items[j..];
            if (std.ascii.startsWithIgnoreCase(line, "XMAS")) {
                try stdout.print("Found horizontal at {d},{d}\n", .{ i, j });
                sum += 1;
            } else if (std.ascii.startsWithIgnoreCase(line, "SAMX")) {
                try stdout.print("Found horizontal at {d},{d}\n", .{ i, j });
                sum += 1;
            }

            // Search vertical (down; up in reverse)
            if (i < (grid.items.len - 3)) {
                vbuf[0] = grid.items[i].items[j];
                vbuf[1] = grid.items[i + 1].items[j];
                vbuf[2] = grid.items[i + 2].items[j];
                vbuf[3] = grid.items[i + 3].items[j];

                if (std.mem.eql(u8, &vbuf, "XMAS") or std.mem.eql(u8, &vbuf, "SAMX")) {
                    try stdout.print("Found vertical at {d},{d}\n", .{ i, j });
                    sum += 1;
                }
            }

            // Search diagonal down right
            if (i < (grid.items.len - 3) and j < (row.items.len - 3)) {
                vbuf[0] = grid.items[i].items[j];
                vbuf[1] = grid.items[i + 1].items[j + 1];
                vbuf[2] = grid.items[i + 2].items[j + 2];
                vbuf[3] = grid.items[i + 3].items[j + 3];

                if (std.mem.eql(u8, &vbuf, "XMAS") or std.mem.eql(u8, &vbuf, "SAMX")) {
                    try stdout.print("Found DR at {d},{d}\n", .{ i, j });
                    sum += 1;
                }
            }

            // Search diagonal down left
            if (i < (grid.items.len - 3) and j >= 3) {
                vbuf[0] = grid.items[i].items[j];
                vbuf[1] = grid.items[i + 1].items[j - 1];
                vbuf[2] = grid.items[i + 2].items[j - 2];
                vbuf[3] = grid.items[i + 3].items[j - 3];

                if (std.mem.eql(u8, &vbuf, "XMAS") or std.mem.eql(u8, &vbuf, "SAMX")) {
                    try stdout.print("Found DL at {d},{d}\n", .{ i, j });
                    sum += 1;
                }
            }
        }

        //try stdout.print("Sum after row {d} is {d}\n", .{ i, sum });
    }

    try stdout.print("Sum is {d}\n", .{sum});
}
