const std = @import("std");
const math = std.math;

const JunctionBox = struct {
    x: isize,
    y: isize,
    z: isize,

    connected_count: usize,
    connected_to: ?*JunctionBox,

    const Self = @This();

    fn from(data: []const u8) !Self {
        var line = std.mem.splitSequence(u8, data, ",");

        const x = try std.fmt.parseInt(isize, line.next().?, 10);
        const y = try std.fmt.parseInt(isize, line.next().?, 10);
        const z = try std.fmt.parseInt(isize, line.next().?, 10);

        return .{
            .x = x,
            .y = y,
            .z = z,
            .connected_count = 1,
            .connected_to = null,
        };
    }

    const Distance = struct {
        src_box: *JunctionBox,
        dst_box: *JunctionBox,
        distance: isize,
    };

    fn distanceTo(self: *Self, other: *JunctionBox) Distance {
        const dx = math.pow(isize, self.x - other.x, 2);
        const dy = math.pow(isize, self.y - other.y, 2);
        const dz = math.pow(isize, self.z - other.z, 2);

        return .{
            .src_box = self,
            .dst_box = other,
            .distance = dx + dy + dz,
        };
    }

    fn getRoot(self: *Self) *JunctionBox {
        if (self.connected_to) |parent| {
            return parent.getRoot();
        }
        return self;
    }

    fn join(self: *Self, other: *JunctionBox) void {
        var self_root = self.getRoot();
        var other_root = other.getRoot();

        if (self_root == other_root) return;

        // Ref: https://en.wikipedia.org/wiki/Disjoint-set_data_structure#Union_by_size
        if (self_root.connected_count > other_root.connected_count) {
            other_root.connected_to = self_root;
            self_root.connected_count += other_root.connected_count;
        } else {
            self_root.connected_to = other_root;
            other_root.connected_count += self_root.connected_count;
        }
    }
};

fn createOpenCircuit(data: []const u8, allocator: std.mem.Allocator) !std.ArrayList(JunctionBox) {
    var lines = std.mem.splitSequence(u8, data, "\n");
    var boxes = std.ArrayList(JunctionBox).empty;

    while (lines.next()) |line| {
        try boxes.append(allocator, try JunctionBox.from(line));
    }

    return boxes;
}

fn OrderedConnections() type {
    const ordering = struct {
        fn lessThan(context: void, a: JunctionBox.Distance, b: JunctionBox.Distance) math.Order {
            _ = context;
            return math.order(a.distance, b.distance);
        }
    };
    return std.PriorityQueue(JunctionBox.Distance, void, ordering.lessThan);
}

fn enumeratePossibleConnections(circuit: *std.ArrayList(JunctionBox), allocator: std.mem.Allocator) !OrderedConnections() {
    var q = OrderedConnections().init(allocator, {});
    const num_boxes = circuit.items.len;

    for (0..num_boxes - 1) |src_index| {
        const src_box: *JunctionBox = &circuit.items[src_index];
        for (src_index + 1..num_boxes) |dst_index| {
            const dst_box: *JunctionBox = &circuit.items[dst_index];
            try q.add(src_box.distanceTo(dst_box));
        }
    }

    return q;
}

fn getNBiggestCircuitsCombinedSize(circuit: *std.ArrayList(JunctionBox), comptime n: comptime_int, allocator: std.mem.Allocator) !usize {
    const ordering = struct {
        fn greaterThan(context: void, a: usize, b: usize) math.Order {
            _ = context;
            return math.order(a, b).invert();
        }
    };

    var q = std.PriorityQueue(usize, void, ordering.greaterThan).init(allocator, {});
    defer q.deinit();

    for (circuit.items) |box| {
        try q.add(box.connected_count);
    }

    var combined_size: usize = 1;
    for (0..n) |_| {
        combined_size *= q.remove();
    }

    return combined_size;
}

fn isCircuitClosed(circuit: *std.ArrayList(JunctionBox)) bool {
    var seen_root = false;

    for (circuit.items) |box| {
        if (box.connected_to == null) {
            if (seen_root) return false;
            seen_root = true;
        }
    }

    return true;
}

pub fn main() !void {
    const data = @embedFile("input");

    var area = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer area.deinit();

    const allocator = area.allocator();

    var circuit = try createOpenCircuit(data, allocator);

    var ordered_pairs = try enumeratePossibleConnections(&circuit, allocator);
    var connection_count: usize = 0;

    while (ordered_pairs.removeOrNull()) |pair| {
        pair.src_box.join(pair.dst_box);
        connection_count += 1;

        if (connection_count == 1000) {
            const part_1 = try getNBiggestCircuitsCombinedSize(&circuit, 3, allocator);
            std.debug.print("Part 1: {}\n", .{part_1});
        }

        if (isCircuitClosed(&circuit)) {
            const part_2 = pair.src_box.x * pair.dst_box.x;
            std.debug.print("Part 2: {}\n", .{part_2});
            break;
        }
    }
}
