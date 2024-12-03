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
        defer fields.clearRetainingCapacity(); // Clear fields each loop

        // Array of tuple (range_start, range_len, is_increasing)
        var nums = std.ArrayList(isize).init(alloc);
        defer nums.deinit();

        // Convert whole row to array of numbers
        for (fields.items) |field| {
            const v = std.fmt.parseInt(isize, field, 10) catch break;
            try nums.append(v);
        }

        var short_list = std.ArrayList(isize).init(alloc);
        defer short_list.deinit();

        // Skip first element
        try short_list.appendSlice(nums.items[1..]);

        var j: usize = 0;
        var k: usize = 1;
        var increasing: bool = false;
        var has_safe: bool = true;
        var last_num: isize = nums.items[0]; // Since we skipped first number
        while (j < nums.items.len) : (j += 1) {
            k = 1;
            has_safe = true;
            while (k < short_list.items.len) : (k += 1) {
                const diff = short_list.items[k] - short_list.items[k - 1];

                // If starting new range
                if (k == 1) {
                    if (diff < 0) {
                        increasing = false;
                    } else {
                        increasing = true;
                    }
                }

                // If difference is outside tolerance, end range.
                if ((increasing and (diff < 1 or diff > 3)) or (!increasing and (diff < -3 or diff > -1))) {
                    has_safe = false;
                    break;
                }
            }

            if (has_safe) {
                break;
            }

            // For next loop, append removed number then swap remove with next index.
            try short_list.append(last_num);
            last_num = short_list.swapRemove(j);
        }

        // If removing an item yields safe list, then safe by definition.
        if (has_safe) {
            safe += 1;
        } else {
            // Else try full list
            k = 1;
            has_safe = true;
            while (k < nums.items.len) : (k += 1) {
                const diff = nums.items[k] - nums.items[k - 1];

                // If starting new range
                if (k == 1) {
                    if (diff < 0) {
                        increasing = false;
                    } else {
                        increasing = true;
                    }
                }

                // If difference is outside tolerance, end range.
                if ((increasing and (diff < 1 or diff > 3)) or (!increasing and (diff < -3 or diff > -1))) {
                    has_safe = false;
                    break;
                }
            }

            if (has_safe) {
                safe += 1;
            }
        }
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    try stdout.print("{d} reports are safe.\n", .{safe});
}
