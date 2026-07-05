const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

/// Serializes user-facing output so lines from the cleaning threads don't interleave.
pub const Printer = struct {
    mutex: Io.Mutex = .init,
    file: Io.File,
    io: Io,

    pub fn init(io: Io) Printer {
        return .{ .file = .stdout(), .io = io };
    }

    pub fn print(self: *Printer, comptime fmt: []const u8, args: anytype) void {
        var buffer: [1024]u8 = undefined;
        var writer: Io.Writer = .fixed(&buffer);
        writer.print(fmt, args) catch {};
        const line = writer.buffered();

        self.mutex.lockUncancelable(self.io);
        defer self.mutex.unlock(self.io);
        self.file.writeStreamingAll(self.io, line) catch {};
    }
};

pub fn directoryHasFile(io: Io, dir: Io.Dir, extension: []const u8) bool {
    var it = dir.iterate();
    while (it.next(io) catch return false) |entry| {
        if (entry.kind != .file) continue;
        if (entry.name.len > extension.len and std.mem.endsWith(u8, entry.name, extension)) {
            return true;
        }
    }
    return false;
}

pub fn askYesNoQuestion(io: Io, printer: *Printer, prompt: []const u8, max_retries: usize) bool {
    printer.print("{s} (y/n)\n", .{prompt});

    var stdin_buffer: [256]u8 = undefined;
    var stdin_reader = Io.File.stdin().readerStreaming(io, &stdin_buffer);
    const reader = &stdin_reader.interface;

    var retries: usize = 0;
    while (retries < max_retries) : (retries += 1) {
        const line = (reader.takeDelimiter('\n') catch return false) orelse return false;
        const input = std.mem.trim(u8, line, " \t\r");

        if (std.ascii.eqlIgnoreCase(input, "y") or std.ascii.eqlIgnoreCase(input, "yes")) return true;
        if (std.ascii.eqlIgnoreCase(input, "n") or std.ascii.eqlIgnoreCase(input, "no")) return false;

        printer.print("Invalid input. Please enter 'y' or 'n':\n", .{});
    }

    return false;
}

pub const RemoveDirsError = Allocator.Error ||
    Io.Dir.Iterator.Error ||
    Io.Dir.OpenError ||
    Io.Dir.DeleteTreeError;

/// Walks `dir` recursively. Directories named in `targets` are deleted whole;
/// directories named in `skips` are left for the other cleaning thread and not
/// entered, so the two threads never touch the same subtree.
pub fn removeUnwantedDirectories(
    io: Io,
    gpa: Allocator,
    dir: Io.Dir,
    path: *std.ArrayList(u8),
    targets: []const []const u8,
    skips: []const []const u8,
    printer: *Printer,
) RemoveDirsError!void {
    var it = dir.iterate();
    while (try it.next(io)) |entry| {
        if (entry.kind != .directory) continue;

        if (nameIn(targets, entry.name)) {
            if (path.items.len == 0) {
                printer.print("Removing {s}\n", .{entry.name});
            } else {
                printer.print("Removing {s}{c}{s}\n", .{ path.items, std.fs.path.sep, entry.name });
            }
            try dir.deleteTree(io, entry.name);
        } else if (nameIn(skips, entry.name)) {
            continue;
        } else {
            var sub_dir = dir.openDir(io, entry.name, .{
                .iterate = true,
                .follow_symlinks = false,
            }) catch |err| switch (err) {
                error.FileNotFound, error.NotDir => continue,
                else => return err,
            };
            defer sub_dir.close(io);

            const old_len = path.items.len;
            defer path.shrinkRetainingCapacity(old_len);
            if (path.items.len > 0) try path.append(gpa, std.fs.path.sep);
            try path.appendSlice(gpa, entry.name);

            try removeUnwantedDirectories(io, gpa, sub_dir, path, targets, skips, printer);
        }
    }
}

/// Entries starting with '.' also match as a name suffix, so ".xcworkspace"
/// matches "MyProject (Mac).xcworkspace" as well as a dir literally named so.
fn nameIn(list: []const []const u8, name: []const u8) bool {
    for (list) |item| {
        if (std.mem.eql(u8, item, name)) return true;
        if (item[0] == '.' and name.len > item.len and std.mem.endsWith(u8, name, item)) return true;
    }
    return false;
}

test nameIn {
    const dirs: []const []const u8 = &.{ "Saved", "Binaries", ".vs", ".xcworkspace" };
    try std.testing.expect(nameIn(dirs, "Saved"));
    try std.testing.expect(nameIn(dirs, ".vs"));
    try std.testing.expect(nameIn(dirs, ".xcworkspace"));
    try std.testing.expect(nameIn(dirs, "MyProject (Mac).xcworkspace"));
    try std.testing.expect(!nameIn(dirs, "Content"));
    try std.testing.expect(!nameIn(dirs, "saved"));
    try std.testing.expect(!nameIn(dirs, "xcworkspace"));
}
