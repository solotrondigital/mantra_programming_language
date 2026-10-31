import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;
let outputChannel: vscode.OutputChannel;

export function activate(context: vscode.ExtensionContext) {
    outputChannel = vscode.window.createOutputChannel('Mantra Language Server');
    outputChannel.appendLine('Mantra Language Support extension activating...');

    const serverPath = resolveServerBinary();
    if (!serverPath) {
        const msg = 'Mantra Language Server (mantra-lsp) not found. Please install the Mantra toolchain or set "mantra.languageServerPath" in settings.';
        outputChannel.appendLine(`ERROR: ${msg}`);
        vscode.window.showErrorMessage(msg, 'Open Settings').then(selection => {
            if (selection === 'Open Settings') {
                vscode.commands.executeCommand('workbench.action.openSettings', 'mantra.languageServerPath');
            }
        });
        return;
    }

    outputChannel.appendLine(`Using mantra-lsp binary at: ${serverPath}`);

    function createClient(binPath: string): LanguageClient {
        const serverOptions: ServerOptions = {
            run: {
                command: binPath,
                args: ['--stdio']
            },
            debug: {
                command: binPath,
                args: ['--stdio']
            }
        };

        const clientOptions: LanguageClientOptions = {
            documentSelector: [{ scheme: 'file', language: 'mantra' }],
            synchronize: {
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.mantra')
            },
            outputChannel
        };

        return new LanguageClient(
            'mantra-lsp',
            'Mantra Language Server',
            serverOptions,
            clientOptions
        );
    }

    client = createClient(serverPath);

    client.start().then(() => {
        outputChannel.appendLine('Mantra Language Server successfully started and connected.');
    }).catch(err => {
        outputChannel.appendLine(`Failed to start Mantra Language Server: ${err}`);
        vscode.window.showErrorMessage(`Failed to start Mantra Language Server: ${err}`);
    });

    context.subscriptions.push(
        vscode.commands.registerCommand('mantra.restartLanguageServer', async () => {
            outputChannel.appendLine('Restarting Mantra Language Server...');
            const currentPath = resolveServerBinary();
            if (!currentPath) {
                vscode.window.showErrorMessage('Mantra Language Server binary (mantra-lsp) not found on PATH.');
                return;
            }

            try {
                if (client) {
                    try {
                        await client.stop();
                    } catch (e) {
                        outputChannel.appendLine(`Note during stop: ${e}`);
                    }
                }
                client = createClient(currentPath);
                await client.start();
                outputChannel.appendLine('Mantra Language Server restarted successfully.');
                vscode.window.showInformationMessage('Mantra Language Server restarted successfully.');
            } catch (err) {
                outputChannel.appendLine(`Failed to restart Mantra Language Server: ${err}`);
                vscode.window.showErrorMessage(`Failed to restart Mantra Language Server: ${err}`);
            }
        }),
        vscode.commands.registerCommand('mantra.showOutput', () => {
            outputChannel.show();
        })
    );
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

function resolveServerBinary(): string | undefined {
    const config = vscode.workspace.getConfiguration('mantra');
    const configuredPath = config.get<string>('languageServerPath');

    if (configuredPath && configuredPath !== 'mantra-lsp') {
        if (fs.existsSync(configuredPath)) {
            return configuredPath;
        }
    }

    // Check workspace target folders first (for active Mantra development)
    if (vscode.workspace.workspaceFolders) {
        for (const folder of vscode.workspace.workspaceFolders) {
            const debugBinary = path.join(folder.uri.fsPath, 'target', 'debug', 'mantra-lsp');
            if (fs.existsSync(debugBinary)) {
                return debugBinary;
            }
            const releaseBinary = path.join(folder.uri.fsPath, 'target', 'release', 'mantra-lsp');
            if (fs.existsSync(releaseBinary)) {
                return releaseBinary;
            }
        }
    }

    // Check standard install directories
    const home = os.homedir();
    const candidatePaths = [
        path.join(home, '.mantra', 'bin', 'mantra-lsp'),
        path.join(home, '.cargo', 'bin', 'mantra-lsp'),
        '/usr/local/bin/mantra-lsp',
        '/opt/homebrew/bin/mantra-lsp'
    ];

    for (const p of candidatePaths) {
        if (fs.existsSync(p)) {
            return p;
        }
    }

    // Fallback: check if 'mantra-lsp' is in system PATH
    return 'mantra-lsp';
}

