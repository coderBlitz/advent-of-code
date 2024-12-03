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

fn find_next(buf: []u8, char: u8) ?usize {
    var i: usize = 0;
    for (buf) |c| {
        if (c == char) {
            return i;
        }

        i += 1;
    }

    return null;
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
    var fields = std.ArrayList([]u8).init(alloc);
    var sum: usize = 0;
    defer fields.deinit();
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;
        try stdout.print("Line {d} length: {d}\n", .{ i + 1, line.len });

        // Test split on commas (simple CSV)
        //const n = try split(&fields, line, ',');

        //try stdout.print("Split returned {d} fields.\n", .{n});

        //for (fields.items) |field| {
        //    try stdout.print("Field = {s}\n", .{field});
        //}
        var cursor = line[0..];
        while (find_next(cursor, 'm')) |idx| {
            cursor = cursor[idx..];
            if (std.ascii.startsWithIgnoreCase(cursor[0..4], "mul(")) {
                cursor = cursor[4..];

                // Find closing parenthesis else break to next line.
                if (find_next(cursor, ')')) |end| {
                    const n = try split(&fields, cursor[0..end], ',');
                    defer fields.clearRetainingCapacity();

                    // If not 2 fields exactly, ignore and move on.
                    if (n != 2) {
                        continue;
                    }

                    // Try to parse the two fields.
                    const num1 = std.fmt.parseInt(usize, fields.items[0], 10) catch continue;
                    const num2 = std.fmt.parseInt(usize, fields.items[1], 10) catch continue;

                    //try stdout.print("Found {d} and {d}.\n", .{ num1, num2 });

                    sum += num1 * num2;

                    // Move past closing parenthesis
                    cursor = cursor[end + 1 ..];
                } else {
                    break;
                }
            } else {
                // Skip single 'm' and move forward.
                cursor = cursor[1..];
            }
        }
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    try stdout.print("Sum is {d}\n", .{sum});
}
