const std = @import("std");

const Present = struct { area: usize };

pub fn parsePresents(data: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Present) {
    var presents = std.ArrayList(Present).empty;

    var raw_presents = std.mem.splitSequence(u8, data, "\n\n");
    while (raw_presents.next()) |raw_present| {
        var area: usize = 0;
        for (raw_present) |char| {
            if (char == '.' or char == '#') {
                area += 1;
            }
        }
        try presents.append(allocator, .{ .area = area });
    }

    return presents;
}

pub fn Tree(comptime present_types: usize) type {
    return struct {
        const Self = @This();

        area_under_tree: usize,
        present_counts: [present_types]usize,

        fn fromRegionDescription(region: []const u8) !Self {
            var data = std.mem.splitSequence(u8, region, ":");

            var area_description = std.mem.splitSequence(u8, data.next().?, "x");
            var area_under_tree: usize = 1;
            while (area_description.next()) |digit| {
                area_under_tree *= try std.fmt.parseInt(usize, digit, 10);
            }

            var present_counts: [present_types]usize = undefined;
            var present_index: usize = 0;
            var present_description = std.mem.splitSequence(u8, std.mem.trimStart(u8, data.next().?, " "), " ");
            while (present_description.next()) |digit| {
                present_counts[present_index] = try std.fmt.parseInt(usize, digit, 10);
                present_index += 1;
            }

            return .{ .area_under_tree = area_under_tree, .present_counts = present_counts };
        }

        fn hasEnoughSpaceForPresents(self: *const Self, presents: *const std.ArrayList(Present)) bool {
            var need_space: usize = 0;

            // Surprisingly, there is no need to actually pack the presents. Either they all fit next to each
            // other, or do not fit even if they were perfectly compacted. Read your inputs kids. :^)
            for (self.present_counts, 0..) |present_type_count, present_index| {
                const present = presents.items[present_index];
                need_space += present.area * present_type_count;
            }

            return need_space <= self.area_under_tree;
        }
    };
}

pub fn main() !void {
    const data = @embedFile("input");

    var area = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer area.deinit();

    const allocator = area.allocator();

    var presents_and_trees = std.mem.splitSequence(u8, data, "\n\n\n");

    const presents = try parsePresents(presents_and_trees.next().?, allocator);
    var trees = std.mem.splitSequence(u8, presents_and_trees.next().?, "\n");

    var trees_with_enough_space: usize = 0;
    while (trees.next()) |tree| {
        const t = try Tree(6).fromRegionDescription(tree);
        if (t.hasEnoughSpaceForPresents(&presents)) {
            trees_with_enough_space += 1;
        }
    }

    std.debug.print("Part 1: {d}\n", .{trees_with_enough_space});
}
