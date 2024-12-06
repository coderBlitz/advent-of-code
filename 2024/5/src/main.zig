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

/// Attempt to find the value in the given slice, returning the index if found.
fn find(comptime T: type, slice: []T, val: *T) ?usize {
    var i: usize = 0;
    while (i < slice.len) : (i += 1) {
        if (slice[i] == val) {
            return i;
        }
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

    // The rules
    var pre_rules = std.AutoHashMap(usize, std.ArrayList(usize)).init(alloc);
    defer pre_rules.deinit();

    var buf: [4096]u8 = undefined;
    var i: usize = 0;
    var fields = std.ArrayList([]u8).init(alloc);
    defer fields.deinit();

    // First loop parses the rules
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;
        if (line.len == 0) {
            break;
        }

        // Split the entries
        _ = try split(&fields, line, '|');

        const n1 = try std.fmt.parseInt(usize, fields.items[0], 10);
        const n2 = try std.fmt.parseInt(usize, fields.items[1], 10);

        // Get entry then allocate if it doesn't exist, then append n2.
        const entry = try pre_rules.getOrPut(n1);
        if (!entry.found_existing) {
            entry.value_ptr.* = std.ArrayList(usize).init(alloc);
        }
        try entry.value_ptr.*.append(n2);

        fields.clearRetainingCapacity();
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    // Second loop reads the update pages
    var update_set = std.AutoHashMap(usize, u8).init(alloc);
    defer update_set.deinit();
    var sum: usize = 0;
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;
        if (line.len == 0) {
            break;
        }

        // Split the entries
        _ = try split(&fields, line, ',');

        // Part 1
        // Start with empty hashset.
        // For each number:
        // 1. Check if slice from 0 to previous number is in set, if so rule violated (STOP).
        // 2. If current number is in set, remove it.
        // 3. Insert rules for number to set.
        var j: usize = 0;
        var valid = true;
        while (j < fields.items.len) : (j += 1) {
            const cur = try std.fmt.parseInt(usize, fields.items[j], 10);

            // Check if previous numbers in set
            for (fields.items[0..j]) |v| {
                const num = try std.fmt.parseInt(usize, v, 10);
                if (update_set.getKey(num)) |_| {
                    valid = false;
                    break;
                }
            }

            // Remove current num if in set.
            _ = (update_set.remove(cur));

            // Add rules for cur to set
            if (pre_rules.get(cur)) |arr| {
                for (arr.items) |n| {
                    try update_set.put(n, 0);
                }
            }
        }

        // If valid, get middle entry and sum.
        if (valid) {
            sum += try std.fmt.parseInt(usize, fields.items[fields.items.len / 2], 10);
        }

        fields.clearRetainingCapacity();
        update_set.clearRetainingCapacity();
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    try stdout.print("Sum is {d}\n", .{sum});
}
