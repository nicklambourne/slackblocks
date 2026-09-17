using System;
using System.IO;

namespace Slackblocks.Tests;

/// <summary>Locates the repository so tests can read the shared specification.</summary>
internal static class Repository
{
    public static string Root { get; } = FindRoot();

    public static string PathTo(params string[] parts) => Path.Combine([Root, .. parts]);

    private static string FindRoot()
    {
        for (var directory = new DirectoryInfo(AppContext.BaseDirectory); directory is not null; directory = directory.Parent)
        {
            if (File.Exists(Path.Combine(directory.FullName, "spec", "manifest.json")))
            {
                return directory.FullName;
            }
        }

        throw new InvalidOperationException("Could not find the slackblocks repository root above " + AppContext.BaseDirectory);
    }
}
