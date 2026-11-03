
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

using NuGet.Common;
using NuGet.Configuration;
using NuGet.Protocol;
using NuGet.Protocol.Core.Types;

namespace MantraPackager
{
    internal class Program
    {
        // Chocolatey package push endpoint
        private const string ChocolateySource =
            "https://push.chocolatey.org/";

        // IMPORTANT:
        // Replace this placeholder with your actual Chocolatey API key.
        // Never commit this file with your real API key to GitHub.
        private const string ChocolateyApiKey =
           "1ac46c92-993e-4dba-bf40-b69cfa549dfe";

        private const int PushTimeoutSeconds = 300;

        private static async Task<int> Main(string[] args)
        {
            try
            {
                Console.WriteLine("======================================");
                Console.WriteLine(" Mantra Chocolatey Package Publisher");
                Console.WriteLine("======================================");

                // Resolve package path.
                // Expected structure:
                //
                // choco-mantra/
                //   mantra.nuspec
                //   mantra.0.8.0.nupkg
                //   tools/
                //   mantra-packager/
                //     Program.cs
                //
                string currentDirectory =
                    Directory.GetCurrentDirectory();

                string packageDirectory =
                    Directory.GetParent(currentDirectory)?.FullName
                    ?? throw new DirectoryNotFoundException(
                        "Could not resolve package directory."
                    );

                string packagePath = Path.Combine(
                    packageDirectory,
                    "mantra.0.8.0.nupkg"
                );

                // Validate API key.
                if (string.IsNullOrWhiteSpace(ChocolateyApiKey) ||
                    ChocolateyApiKey.Contains("PASTE_YOUR"))
                {
                    throw new InvalidOperationException(
                        "Please configure your Chocolatey API key in Program.cs."
                    );
                }

                // Validate package.
                if (!File.Exists(packagePath))
                {
                    throw new FileNotFoundException(
                        "Chocolatey package not found.",
                        packagePath
                    );
                }

                FileInfo packageInfo = new FileInfo(packagePath);

                Console.WriteLine();
                Console.WriteLine("Package Details");
                Console.WriteLine("--------------------------------------");
                Console.WriteLine($"Package : {packageInfo.Name}");
                Console.WriteLine($"Path    : {packagePath}");
                Console.WriteLine($"Size    : {packageInfo.Length:N0} bytes");
                Console.WriteLine($"Source  : {ChocolateySource}");
                Console.WriteLine();

                Console.WriteLine("Preparing NuGet repository...");

                var packageSource =
                    new PackageSource(ChocolateySource);

                var repository =
                    Repository.Factory.GetCoreV3(packageSource);

                PackageUpdateResource? pushResource =
                    await repository.GetResourceAsync<PackageUpdateResource>();

                if (pushResource == null)
                {
                    throw new InvalidOperationException(
                        "Unable to initialize NuGet PackageUpdateResource."
                    );
                }

                Console.WriteLine("Repository initialized.");
                Console.WriteLine();

                Console.WriteLine("Uploading package to Chocolatey...");
                Console.WriteLine("Please wait...");
                Console.WriteLine();

                var packageFiles = new List<string>
                {
                    packagePath
                };

            await pushResource.Push(
    packageFiles,
    symbolSource: null,
    timeoutInSecond: PushTimeoutSeconds,
    disableBuffering: false,
    getApiKey: _ => ChocolateyApiKey,
    getSymbolApiKey: _ => null,
    noServiceEndpoint: false,
    skipDuplicate: false,
    symbolPackageUpdateResource: null,
    log: NullLogger.Instance
);
                Console.WriteLine();
                Console.WriteLine("======================================");
                Console.WriteLine(" PACKAGE PUSH COMPLETED");
                Console.WriteLine("======================================");
                Console.WriteLine(
                    "Package upload request completed successfully."
                );
                Console.WriteLine();
                Console.WriteLine("Package: mantra 0.8.0");
                Console.WriteLine(
                    "Check your Chocolatey package dashboard for moderation status."
                );

                return 0;
            }
            catch (Exception ex)
            {
                Console.WriteLine();
                Console.WriteLine("======================================");
                Console.WriteLine(" PACKAGE PUSH FAILED");
                Console.WriteLine("======================================");

                Exception? currentException = ex;
                int level = 0;

                while (currentException != null)
                {
                    Console.WriteLine();
                    Console.WriteLine($"Error Level: {level}");
                    Console.WriteLine(
                        $"Type   : {currentException.GetType().FullName}"
                    );
                    Console.WriteLine(
                        $"Message: {currentException.Message}"
                    );

                    if (currentException is System.Net.Http.HttpRequestException httpEx &&
                        httpEx.StatusCode.HasValue)
                    {
                        Console.WriteLine(
                            $"HTTP Status: {(int)httpEx.StatusCode.Value} " +
                            $"{httpEx.StatusCode.Value}"
                        );
                    }

                    currentException = currentException.InnerException;
                    level++;
                }

                Console.WriteLine();
                Console.WriteLine("Check the following:");
                Console.WriteLine("1. Chocolatey API key is valid.");
                Console.WriteLine("2. Package ID and version are correct.");
                Console.WriteLine("3. Your account is authorized to push this package.");
                Console.WriteLine("4. The package is not blocked by moderation.");
                Console.WriteLine();

                return 1;
            }
        }
    }
}
