const std = @import("std");
const Io = std.Io;

const zigport = @import("zigport");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    const args = try init.minimal.args.toSlice(arena);

    if (args.len < 2) {
        printUsage(init.io);
        std.process.exit(2);
    }

    const command = args[1];
    if (std.mem.eql(u8, command, "clean")) {
        try zigport.unreal.clean(init.io, init.gpa);
    } else {
        printUsage(init.io);
        std.process.exit(2);
    }
}

fn printUsage(io: Io) void {
    var stdout_buffer: [256]u8 = undefined;
    var stdout_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout = &stdout_writer.interface;

    stdout.print(
        \\Usage: zrrp <command>
        \\
        \\Commands:
        \\  clean    Remove Unreal build artifact directories
        \\           (Saved, Intermediate, Binaries, DerivedDataCache, .idea, .vs)
        \\
    , .{}) catch {};
    stdout.flush() catch {};
}
