const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const libmath = b.addSharedLibrary(.{
        .name = "math_zig",
        .root_source_file = b.path("src/math.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(libmath);
}
