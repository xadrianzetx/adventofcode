const std = @import("std");

const MatchesTag = enum { none, partial, full };
const Matches = union(MatchesTag) { none: void, partial: usize, full: usize };

pub fn inspectRange(lower: usize, upper: usize) !struct { usize, usize } {
    var partial_matches: usize = 0;
    var full_matches: usize = 0;

    for (lower..upper + 1) |id| {
        switch (try inspectId(id)) {
            .full => |matched_id| {
                full_matches += matched_id;
                partial_matches += matched_id;
            },
            .partial => |matched_id| partial_matches += matched_id,
            .none => {},
        }
    }

    return .{ partial_matches, full_matches };
}

pub fn inspectId(id: usize) !Matches {
    var buf: [32]u8 = undefined;
    const id_str = try std.fmt.bufPrint(&buf, "{}", .{id});

    const max_sequence_size = id_str.len / 2;
    var matches = Matches{ .none = {} };

    for (1..max_sequence_size + 1) |sequence_size| {
        if (sequenceRepeats(id_str, sequence_size)) {
            if ((sequence_size * 2) == id_str.len) {
                matches = .{ .full = id };
            } else {
                matches = .{ .partial = id };
            }
        }
    }

    return matches;
}

pub fn sequenceRepeats(data: []const u8, sequence_size: usize) bool {
    var window = std.mem.window(u8, data, sequence_size, sequence_size);
    var current = window.next().?;

    while (window.next()) |next| {
        if (!std.mem.eql(u8, current, next)) {
            return false;
        }
        current = next;
    }
    return true;
}

pub fn main() !void {
    const data = @embedFile("input");

    var lines = std.mem.splitSequence(u8, data, ",");

    var partial_matches: usize = 0;
    var full_matches: usize = 0;

    while (lines.next()) |line| {
        var raw_range = std.mem.splitSequence(u8, line, "-");

        const lower = try std.fmt.parseInt(usize, raw_range.next().?, 10);
        const upper = try std.fmt.parseInt(usize, raw_range.next().?, 10);

        const matches = try inspectRange(lower, upper);

        partial_matches += matches.@"0";
        full_matches += matches.@"1";
    }

    std.debug.print("Part 1: {d}\n", .{full_matches});
    std.debug.print("Part 2: {d}\n", .{partial_matches});
}
