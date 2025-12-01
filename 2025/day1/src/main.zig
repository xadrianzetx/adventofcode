const std = @import("std");

const ParsingError = error{UnknownDirection};

const RotationTag = enum { L, R };
const Rotation = union(RotationTag) {
    L: i32,
    R: i32,

    fn decode(line: []const u8) !Rotation {
        const direction = std.meta.stringToEnum(RotationTag, line[0..1]) orelse {
            return ParsingError.UnknownDirection;
        };
        const distance = try std.fmt.parseInt(i32, line[1..], 10);

        switch (direction) {
            .L => return .{ .L = distance },
            .R => return .{ .R = distance },
        }
    }
};

fn Dial(comptime initialPosition: i32) type {
    return struct {
        const Self = @This();

        position: i32,
        leftAtOriginCount: usize,
        passedOriginCount: usize,

        pub fn init() Self {
            return .{ .position = initialPosition, .leftAtOriginCount = 0, .passedOriginCount = 0 };
        }

        pub fn rotate(self: *Self, rotation: Rotation) void {
            switch (rotation) {
                .L => |steps| {
                    if (self.position == 0) {
                        self.passedOriginCount -= 1;
                    }

                    self.passedOriginCount += @intCast(@abs(@divFloor(self.position - steps, 100)));
                    self.position = @mod(self.position - steps, 100);

                    if (self.position == 0) {
                        self.passedOriginCount += 1;
                    }
                },
                .R => |steps| {
                    self.passedOriginCount += @intCast(@divFloor(self.position + steps, 100));
                    self.position = @mod(self.position + steps, 100);
                },
            }

            if (self.position == 0) {
                self.leftAtOriginCount += 1;
            }
        }
    };
}

pub fn main() !void {
    const data = @embedFile("input");

    var lines = std.mem.splitSequence(u8, data, "\n");
    var dial = Dial(50).init();

    while (lines.next()) |line| {
        const rotation = try Rotation.decode(line);
        dial.rotate(rotation);
    }

    std.debug.print("Part 1: {d}\n", .{dial.leftAtOriginCount});
    std.debug.print("Part 2: {d}\n", .{dial.passedOriginCount});
}
