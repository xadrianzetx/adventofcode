const std = @import("std");
const math = std.math;

pub fn readBank(bank: []const u8, allocator: std.mem.Allocator) !std.ArrayList(usize) {
    var bank_buff = std.ArrayList(usize).empty;
    var window = std.mem.window(u8, bank, 1, 1);

    while (window.next()) |battery| {
        try bank_buff.append(allocator, try std.fmt.parseInt(usize, battery, 10));
    }

    return bank_buff;
}

pub fn findMaxTotalJoltage(bank: *const std.ArrayList(usize), batteries_needed: usize) usize {
    var current_battery_index: usize = 0;
    var total_joltage: usize = 0;

    for (0..batteries_needed) |battery_index| {
        const last_visible_battery_index = bank.items.len - (batteries_needed - battery_index - 1);
        const bank_slice = bank.items[current_battery_index..last_visible_battery_index];

        const local_maximum_joltage = naiveMaxInSlice(bank_slice);
        current_battery_index += local_maximum_joltage.@"1" + 1;
        total_joltage += local_maximum_joltage.@"0" * math.pow(usize, 10, (batteries_needed - battery_index - 1));
    }

    return total_joltage;
}

pub fn naiveMaxInSlice(slice: []const usize) struct { usize, usize } {
    var max: usize = 0;
    var idx: usize = 0;

    for (slice, 0..) |item, index| {
        if (item > max) {
            max = item;
            idx = index;
        }
    }

    return .{ max, idx };
}

pub fn main() !void {
    const data = @embedFile("input");

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    var banks = std.mem.splitSequence(u8, data, "\n");

    var total_small_joltage: usize = 0;
    var total_big_joltage: usize = 0;

    while (banks.next()) |raw_bank| {
        const bank = try readBank(raw_bank, allocator);

        total_small_joltage += findMaxTotalJoltage(&bank, 2);
        total_big_joltage += findMaxTotalJoltage(&bank, 12);
    }

    std.debug.print("Part 1: {d}\n", .{total_small_joltage});
    std.debug.print("Part 2: {d}\n", .{total_big_joltage});
}
