const std = @import("std");

const MatchesTag = enum { none, partial, full };
const Matches = union(MatchesTag) { none: void, partial: usize, full: usize };

pub fn inspectRange(lower: usize, upper: usize) !struct { usize, usize } {
    var partialMatches: usize = 0;
    var fullMatches: usize = 0;

    for (lower..upper + 1) |id| {
        switch (try inspectId(id)) {
            .full => |matchedId| {
                fullMatches += matchedId;
                partialMatches += matchedId;
            },
            .partial => |matchedId| partialMatches += matchedId,
            .none => {},
        }
    }

    return .{ partialMatches, fullMatches };
}

pub fn inspectId(id: usize) !Matches {
    var buf: [256]u8 = undefined;
    const idStr = try std.fmt.bufPrint(&buf, "{}", .{id});

    const maxSequenceSize = idStr.len / 2;
    var matches = Matches{ .none = {} };

    for (1..maxSequenceSize + 1) |sequenceSize| {
        if (sequenceRepeats(idStr, sequenceSize)) {
            if ((sequenceSize * 2) == idStr.len) {
                matches = .{ .full = id };
            } else {
                matches = .{ .partial = id };
            }
        }
    }

    return matches;
}

pub fn sequenceRepeats(data: []const u8, sequenceSize: usize) bool {
    var window = std.mem.window(u8, data, sequenceSize, sequenceSize);
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

    var partialMatches: usize = 0;
    var fullMatches: usize = 0;

    while (lines.next()) |line| {
        var rawRange = std.mem.splitSequence(u8, line, "-");

        const lower = try std.fmt.parseInt(usize, rawRange.next().?, 10);
        const upper = try std.fmt.parseInt(usize, rawRange.next().?, 10);

        const matches = try inspectRange(lower, upper);

        partialMatches += matches.@"0";
        fullMatches += matches.@"1";
    }

    std.debug.print("Part 1: {d}\n", .{fullMatches});
    std.debug.print("Part 2: {d}\n", .{partialMatches});
}
