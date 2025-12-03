const std = @import("std");
const math = std.math;

pub fn readBank(bank: []const u8, allocator: std.mem.Allocator) !std.ArrayList(usize) {
    var bankBuff = std.ArrayList(usize).empty;
    var window = std.mem.window(u8, bank, 1, 1);

    while (window.next()) |battery| {
        try bankBuff.append(allocator, try std.fmt.parseInt(usize, battery, 10));
    }

    return bankBuff;
}

pub fn findMaxTotalJoltage(bank: *const std.ArrayList(usize), batteriesNeeded: usize) !usize {
    var currentBatteryIndex: usize = 0;
    var totalJoltage: usize = 0;

    for (0..batteriesNeeded) |batteryIndex| {
        const lastVisibleBatteryIndex = bank.items.len - (batteriesNeeded - batteryIndex - 1);
        const bankSlice = bank.items[currentBatteryIndex..lastVisibleBatteryIndex];

        const localMaximumJoltage = naiveMaxInSlice(bankSlice);
        currentBatteryIndex += localMaximumJoltage.@"1" + 1;
        totalJoltage += localMaximumJoltage.@"0" * math.pow(usize, 10, (batteriesNeeded - batteryIndex - 1));
    }

    return totalJoltage;
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

    var totalSmallJoltage: usize = 0;
    var totalBigJoltage: usize = 0;

    while (banks.next()) |rawBank| {
        const bank = try readBank(rawBank, allocator);

        totalSmallJoltage += try findMaxTotalJoltage(&bank, 2);
        totalBigJoltage += try findMaxTotalJoltage(&bank, 12);
    }

    std.debug.print("Part 1: {d}\n", .{totalSmallJoltage});
    std.debug.print("Part 2: {d}\n", .{totalBigJoltage});
}
