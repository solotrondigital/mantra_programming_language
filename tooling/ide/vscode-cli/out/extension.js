"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = require("vscode");
const cp = require("child_process");
let mantraTerminal;
function activate(context) {
    const outputChannel = vscode.window.createOutputChannel('Mantra CLI');
    outputChannel.appendLine('Mantra CLI & Project Manager extension activated.');
    // Status Bar Item
    const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBar.text = '$(terminal) Mantra CLI';
    statusBar.tooltip = 'Click to open Mantra CLI Actions menu';
    statusBar.command = 'mantra.cli.menu';
    statusBar.show();
    context.subscriptions.push(statusBar);
    // Helper: Run Mantra Command in Integrated Terminal
    function runMantraCommand(subcommand, args = []) {
        const config = vscode.workspace.getConfiguration('mantra');
        const cliPath = config.get('cliPath') || 'mantra';
        const autoShow = config.get('autoShowTerminal') !== false;
        if (!mantraTerminal || mantraTerminal.exitStatus !== undefined) {
            mantraTerminal = vscode.window.createTerminal({
                name: 'Mantra CLI',
                iconPath: new vscode.ThemeIcon('terminal')
            });
        }
        if (autoShow) {
            mantraTerminal.show();
        }
        const fullCommand = [cliPath, subcommand, ...args].join(' ');
        mantraTerminal.sendText(fullCommand);
    }
    // Command: Quick Menu
    const menuCmd = vscode.commands.registerCommand('mantra.cli.menu', async () => {
        const items = [
            { label: '$(heart) Check Toolchain Health', description: 'mantra doctor', detail: 'Diagnose compiler, LSP, mobile tools and dependencies' },
            { label: '$(plus) Create New Project', description: 'mantra create', detail: 'Scaffold frontend hybrid or backend services' },
            { label: '$(play) Run Application (Default)', description: 'mantra run', detail: 'Run app on default device target' },
            { label: '$(globe) Run on Web', description: 'mantra run -d web', detail: 'Run WebAssembly target in browser' },
            { label: '$(device-mobile) Run on Android', description: 'mantra run -d android', detail: 'Run on Android emulator or connected device' },
            { label: '$(device-mobile) Run on iOS', description: 'mantra run -d ios', detail: 'Run on iOS simulator or device' },
            { label: '$(package) Build WebAssembly Bundle', description: 'mantra build web', detail: 'Compile wasm32 bundle' },
            { label: '$(package) Build Android APK', description: 'mantra build apk', detail: 'Compile release APK' },
            { label: '$(package) Build Android App Bundle', description: 'mantra build appbundle', detail: 'Compile AAB for Play Store' },
            { label: '$(package) Build iOS App', description: 'mantra mobile build ios', detail: 'Prepare iOS app bundle' },
            { label: '$(check) Check Code (Parse & Type Check)', description: 'mantra check', detail: 'Fast verification without emitting code' },
            { label: '$(pass) Run Test Suite', description: 'mantra test', detail: 'Discover and execute tests' },
            { label: '$(shield) Mobile Security & Bridge Test', description: 'mantra mobile test', detail: 'Run security and capability conformance' },
            { label: '$(wand) Format Source Code', description: 'mantra fmt', detail: 'Format files using official 4-space rules' },
            { label: '$(search) Lint Code', description: 'mantra lint', detail: 'Static diagnostics on source files' },
            { label: '$(dashboard) Run Benchmarks', description: 'mantra bench', detail: 'Micro-benchmarks with throughput analysis' },
            { label: '$(list-unordered) List Target Architectures', description: 'mantra target list', detail: 'List all supported compilation targets' }
        ];
        const selection = await vscode.window.showQuickPick(items, {
            placeHolder: 'Select a Mantra CLI command to execute'
        });
        if (!selection)
            return;
        switch (selection.description) {
            case 'mantra doctor':
                vscode.commands.executeCommand('mantra.cli.doctor');
                break;
            case 'mantra create':
                vscode.commands.executeCommand('mantra.cli.create');
                break;
            case 'mantra run':
                vscode.commands.executeCommand('mantra.cli.run');
                break;
            case 'mantra run -d web':
                vscode.commands.executeCommand('mantra.cli.runWeb');
                break;
            case 'mantra run -d android':
                vscode.commands.executeCommand('mantra.cli.runAndroid');
                break;
            case 'mantra run -d ios':
                vscode.commands.executeCommand('mantra.cli.runIos');
                break;
            case 'mantra build web':
                vscode.commands.executeCommand('mantra.cli.buildWeb');
                break;
            case 'mantra build apk':
                vscode.commands.executeCommand('mantra.cli.buildApk');
                break;
            case 'mantra build appbundle':
                vscode.commands.executeCommand('mantra.cli.buildAppBundle');
                break;
            case 'mantra mobile build ios':
                vscode.commands.executeCommand('mantra.cli.buildIos');
                break;
            case 'mantra check':
                vscode.commands.executeCommand('mantra.cli.check');
                break;
            case 'mantra test':
                vscode.commands.executeCommand('mantra.cli.test');
                break;
            case 'mantra mobile test':
                vscode.commands.executeCommand('mantra.cli.mobileTest');
                break;
            case 'mantra fmt':
                vscode.commands.executeCommand('mantra.cli.format');
                break;
            case 'mantra lint':
                vscode.commands.executeCommand('mantra.cli.lint');
                break;
            case 'mantra bench':
                vscode.commands.executeCommand('mantra.cli.bench');
                break;
            case 'mantra target list':
                vscode.commands.executeCommand('mantra.cli.targetList');
                break;
        }
    });
    // Command: Create New Project
    const createCmd = vscode.commands.registerCommand('mantra.cli.create', async () => {
        const projectType = await vscode.window.showQuickPick([
            { label: 'Frontend Hybrid Application', description: 'frontend', detail: 'Multi-platform iOS, Android, and Web application' },
            { label: 'Backend REST Service', description: 'backend-rest', detail: 'High-performance HTTP REST API service' },
            { label: 'Backend GraphQL Service', description: 'backend-graphql', detail: 'Schema-driven GraphQL server' }
        ], { placeHolder: 'Select project architecture' });
        if (!projectType)
            return;
        const projectName = await vscode.window.showInputBox({
            prompt: 'Enter your project name (e.g. my_mantra_app)',
            placeHolder: 'my_mantra_app',
            validateInput: (text) => text.trim().length === 0 ? 'Project name cannot be empty' : null
        });
        if (!projectName)
            return;
        if (projectType.description === 'frontend') {
            runMantraCommand('create', ['--type', 'frontend', '--project-name', projectName.trim(), '--platforms', 'ios,android,web']);
        }
        else if (projectType.description === 'backend-rest') {
            runMantraCommand('create', ['--type', 'backend', '--service', 'REST', '--project-name', projectName.trim()]);
        }
        else if (projectType.description === 'backend-graphql') {
            runMantraCommand('create', ['--type', 'backend', '--service', 'GRAPHQL', '--project-name', projectName.trim()]);
        }
    });
    // Register all standard commands
    context.subscriptions.push(menuCmd, createCmd, vscode.commands.registerCommand('mantra.cli.doctor', () => runMantraCommand('doctor')), vscode.commands.registerCommand('mantra.cli.run', () => runMantraCommand('run')), vscode.commands.registerCommand('mantra.cli.runWeb', () => runMantraCommand('run', ['-d', 'web'])), vscode.commands.registerCommand('mantra.cli.runAndroid', () => runMantraCommand('run', ['-d', 'android'])), vscode.commands.registerCommand('mantra.cli.runIos', () => runMantraCommand('run', ['-d', 'ios'])), vscode.commands.registerCommand('mantra.cli.buildWeb', () => runMantraCommand('build', ['web'])), vscode.commands.registerCommand('mantra.cli.buildApk', () => runMantraCommand('build', ['apk'])), vscode.commands.registerCommand('mantra.cli.buildAppBundle', () => runMantraCommand('build', ['appbundle'])), vscode.commands.registerCommand('mantra.cli.buildIos', () => runMantraCommand('mobile', ['build', 'ios'])), vscode.commands.registerCommand('mantra.cli.test', () => runMantraCommand('test')), vscode.commands.registerCommand('mantra.cli.mobileTest', () => runMantraCommand('mobile', ['test'])), vscode.commands.registerCommand('mantra.cli.check', () => runMantraCommand('check')), vscode.commands.registerCommand('mantra.cli.format', () => runMantraCommand('fmt')), vscode.commands.registerCommand('mantra.cli.lint', () => runMantraCommand('lint')), vscode.commands.registerCommand('mantra.cli.bench', () => runMantraCommand('bench')), vscode.commands.registerCommand('mantra.cli.targetList', () => runMantraCommand('target', ['list'])));
    // Sidebar: Mantra Commands Tree View
    const commandsProvider = new MantraCommandsProvider();
    vscode.window.registerTreeDataProvider('mantraCommandsView', commandsProvider);
    // Sidebar: Mantra Doctor Tree View
    const doctorProvider = new MantraDoctorProvider();
    vscode.window.registerTreeDataProvider('mantraDoctorView', doctorProvider);
    vscode.commands.registerCommand('mantra.cli.refreshDoctor', () => doctorProvider.refresh());
}
function deactivate() {
    if (mantraTerminal) {
        mantraTerminal.dispose();
    }
}
// Tree Item for Commands View
class CommandItem extends vscode.TreeItem {
    constructor(label, commandId, icon, tooltipText) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.label = label;
        this.commandId = commandId;
        this.icon = icon;
        this.tooltipText = tooltipText;
        this.tooltip = tooltipText;
        this.iconPath = new vscode.ThemeIcon(icon);
        this.command = {
            command: commandId,
            title: label
        };
    }
}
class MantraCommandsProvider {
    getTreeItem(element) {
        return element;
    }
    getChildren() {
        return Promise.resolve([
            new CommandItem('Toolchain Health Check', 'mantra.cli.doctor', 'heart', 'Run mantra doctor'),
            new CommandItem('Create New Project...', 'mantra.cli.create', 'plus', 'Scaffold a new Mantra project'),
            new CommandItem('Run Application', 'mantra.cli.run', 'play', 'Run app on default device'),
            new CommandItem('Run on Web (Browser)', 'mantra.cli.runWeb', 'globe', 'Run WebAssembly web build'),
            new CommandItem('Run on Android', 'mantra.cli.runAndroid', 'device-mobile', 'Run on Android emulator/device'),
            new CommandItem('Run on iOS', 'mantra.cli.runIos', 'device-mobile', 'Run on iOS simulator/device'),
            new CommandItem('Build WebAssembly', 'mantra.cli.buildWeb', 'package', 'Compile wasm32 package'),
            new CommandItem('Build Android APK', 'mantra.cli.buildApk', 'package', 'Build release APK'),
            new CommandItem('Build Android App Bundle', 'mantra.cli.buildAppBundle', 'package', 'Build AAB bundle'),
            new CommandItem('Build iOS Package', 'mantra.cli.buildIos', 'package', 'Build iOS app bundle'),
            new CommandItem('Type Check Source', 'mantra.cli.check', 'check', 'Run mantra check'),
            new CommandItem('Run Test Suite', 'mantra.cli.test', 'pass', 'Run mantra test'),
            new CommandItem('Format Code', 'mantra.cli.format', 'wand', 'Format code with mantra fmt'),
            new CommandItem('Lint Source Code', 'mantra.cli.lint', 'search', 'Run mantra lint')
        ]);
    }
}
class MantraDoctorProvider {
    constructor() {
        this._onDidChangeTreeData = new vscode.EventEmitter();
        this.onDidChangeTreeData = this._onDidChangeTreeData.event;
    }
    refresh() {
        this._onDidChangeTreeData.fire();
    }
    getTreeItem(element) {
        return element;
    }
    getChildren() {
        return new Promise((resolve) => {
            cp.exec('mantra doctor', (err, stdout) => {
                if (err) {
                    const item = new vscode.TreeItem('Mantra Doctor: error running CLI', vscode.TreeItemCollapsibleState.None);
                    item.iconPath = new vscode.ThemeIcon('error');
                    resolve([item]);
                    return;
                }
                const lines = stdout.split('\n');
                const items = [];
                for (const line of lines) {
                    const trimmed = line.trim();
                    if (trimmed.startsWith('[✓]')) {
                        const item = new vscode.TreeItem(trimmed.replace('[✓]', '').trim(), vscode.TreeItemCollapsibleState.None);
                        item.iconPath = new vscode.ThemeIcon('check', new vscode.ThemeColor('testing.iconPassed'));
                        items.push(item);
                    }
                    else if (trimmed.startsWith('[!]') || trimmed.startsWith('[x]')) {
                        const item = new vscode.TreeItem(trimmed.substring(3).trim(), vscode.TreeItemCollapsibleState.None);
                        item.iconPath = new vscode.ThemeIcon('warning', new vscode.ThemeColor('testing.iconFailed'));
                        items.push(item);
                    }
                }
                if (items.length === 0) {
                    const item = new vscode.TreeItem('All components operational', vscode.TreeItemCollapsibleState.None);
                    item.iconPath = new vscode.ThemeIcon('check');
                    items.push(item);
                }
                resolve(items);
            });
        });
    }
}
//# sourceMappingURL=extension.js.map