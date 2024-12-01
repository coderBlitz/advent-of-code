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

    //var alloc_buf = std.mem.zeroes([2048]u8);
    //var buf_alloc = std.heap.FixedBufferAllocator.init(&alloc_buf);
    //const alloc = buf_alloc.allocator();
    const alloc = std.heap.page_allocator;
    //defer alloc.deinit();

    var buf: [4096]u8 = undefined;
    var i: usize = 0;
    var fields = std.ArrayList([]u8).init(alloc);
    var list1 = std.ArrayList(isize).init(alloc);
    var list2 = std.ArrayList(isize).init(alloc);
    defer fields.deinit();
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;
        //try stdout.print("Line {d} length: {d}\n", .{ i + 1, line.len });

        // Split on the space
        _ = try split(&fields, line, ' ');

        //try stdout.print("Split returned {d} fields.\n", .{n});

        const num1 = try std.fmt.parseInt(isize, fields.items[0], 10);
        const num2 = try std.fmt.parseInt(isize, fields.items[fields.items.len - 1], 10);
        try list1.append(num1);
        try list2.append(num2);
        // for (fields.items) |field| {
        //     try stdout.print("Field = {s}\n", .{field});
        // }

        fields.clearRetainingCapacity();
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    // Sort lists
    std.sort.heap(isize, list1.items, {}, std.sort.asc(isize));
    std.sort.heap(isize, list2.items, {}, std.sort.asc(isize));

    // Diff and sum lists
    i = 0;
    var sum: usize = 0;
    while (i < list1.items.len) : (i += 1) {
        sum += @abs(list1.items[i] - list2.items[i]);
    }

    try stdout.print("Sum is {d}", .{sum});

    // --- PART 2 ---
    // Create hashmap
    var count = std.hash_map.AutoHashMap(isize, usize).init(alloc);

    for (list2.items) |num| {
        const entry = try count.getOrPut(num);
        if (entry.found_existing) {
            entry.value_ptr.* += 1;
        } else {
            entry.value_ptr.* = 1;
        }
    }
}
