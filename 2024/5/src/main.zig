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
    var sum: usize = 0;
    while (reader.readUntilDelimiterOrEof(&buf, '\n')) |maybe_line| : (i += 1) {
        const line = maybe_line orelse break;
        if (line.len == 0) {
            break;
        }

        // Split the entries
        _ = try split(&fields, line, ',');

        var nums = std.ArrayList(usize).init(alloc);
        defer nums.deinit();

        for (fields.items) |field| {
            try nums.append(try std.fmt.parseInt(usize, field, 10));
        }

        // Part 2
        // Start with 2 empty hashsets.
        // First set is of the update rule numbers (everything on this line).
        // Second set is of the combined rules from everything in the first set.
        // Whichever number from the first set is not in the second set goes first.
        // Repeat process with remaining numbers. Find whichever number in first set is not in second, place next.
        // etc.
        // NOTE: Can probably use the u8 in hashmap to count how many times a number is a rule to avoid
        //        needless reconstruction every time by subtracting till 0 then removing.
        var update_set = std.AutoHashMap(usize, u8).init(alloc);
        defer update_set.deinit();
        var j: isize = 0;
        const valid = false;
        while (j < nums.items.len) : (j += 1) {}

        // If invalid, get middle entry and sum.
        if (!valid) {
            sum += try std.fmt.parseInt(usize, fields.items[fields.items.len / 2], 10);
        }

        try stdout.print("Final row: ", .{});
        for (nums.items) |num| {
            try stdout.print("{d} ", .{num});
        }
        try stdout.print("\n", .{});

        fields.clearRetainingCapacity();
        update_set.clearRetainingCapacity();
    } else |_| {
        try stdout.print("Error\n", .{});
    }

    try stdout.print("Invalid sum is {d}\n", .{sum});
}
