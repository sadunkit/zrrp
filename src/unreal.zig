const std = @import("std");
const Io = std.Io;

const utils = @import("utils.zig");

const intermediate_dirs: []const []const u8 = &.{"Intermediate"};
const other_dirs: []const []const u8 = &.{ "Saved", "Binaries", "DerivedDataCache", ".idea", ".vs", ".xcworkspace" };

pub fn clean(io: Io, gpa: std.mem.Allocator) !void {
    var printer = utils.Printer.init(io);

    {
        var cwd = try Io.Dir.cwd().openDir(io, ".", .{ .iterate = true });
        defer cwd.close(io);

        if (!utils.directoryHasFile(io, cwd, ".uproject")) {
            if (!utils.askYesNoQuestion(io, &printer, "This folder doesn't have .uproject file Continue? ", 4)) {
                printer.print("Exiting...\n", .{});
                std.process.exit(0);
            }
        }
    }

    // One thread deletes only Intermediate directories, the other handles the
    // rest. Each skips the other's targets so their subtrees never overlap.
    var intermediate_failed = false;
    var other_failed = false;

    const intermediate_thread = try std.Thread.spawn(.{}, cleanWorker, .{
        io, gpa, intermediate_dirs, other_dirs, &printer, &intermediate_failed,
    });
    const other_thread = try std.Thread.spawn(.{}, cleanWorker, .{
        io, gpa, other_dirs, intermediate_dirs, &printer, &other_failed,
    });

    intermediate_thread.join();
    other_thread.join();

    if (intermediate_failed or other_failed) {
        std.process.exit(1);
    }

    printer.print("Successfully removed directories\n", .{});
}

fn cleanWorker(
    io: Io,
    gpa: std.mem.Allocator,
    targets: []const []const u8,
    skips: []const []const u8,
    printer: *utils.Printer,
    failed: *bool,
) void {
    // Each thread needs its own handle: the directory iterator cursor lives on
    // the file descriptor.
    var dir = Io.Dir.cwd().openDir(io, ".", .{
        .iterate = true,
        .follow_symlinks = false,
    }) catch |err| {
        printer.print("Error: failed to open current directory: {s}\n", .{@errorName(err)});
        failed.* = true;
        return;
    };
    defer dir.close(io);

    var path: std.ArrayList(u8) = .empty;
    defer path.deinit(gpa);

    utils.removeUnwantedDirectories(io, gpa, dir, &path, targets, skips, printer) catch |err| {
        printer.print("Error: {s}\n", .{@errorName(err)});
        failed.* = true;
    };
}
