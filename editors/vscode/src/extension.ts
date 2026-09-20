import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';
import * as https from 'https';
import { exec, execSync } from 'child_process';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Trace
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
    const outputChannel = vscode.window.createOutputChannel('GLSL Extended');
    outputChannel.appendLine('[GLSL Extended] Activating extension...');

    context.subscriptions.push(
        vscode.commands.registerCommand('glsl-extended.restartServer', async () => {
            outputChannel.appendLine('[GLSL Extended] Restarting Language Server...');
            if (client) {
                await client.stop();
                client = undefined;
            }
            await startLanguageServer(context, outputChannel);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('glsl-extended.downloadServer', async () => {
            try {
                await downloadGlslValidator(context, outputChannel, true);
                vscode.window.showInformationMessage('GLSL Validator language server binary updated successfully.');
                vscode.commands.executeCommand('glsl-extended.restartServer');
            } catch (err: any) {
                vscode.window.showErrorMessage(`Failed to update GLSL Validator: ${err?.message || err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('glsl-extended.downloadGlslang', async () => {
            try {
                await downloadGlslang(context, outputChannel);
                vscode.window.showInformationMessage('glslangValidator reference compiler downloaded successfully.');
                vscode.commands.executeCommand('glsl-extended.restartServer');
            } catch (err: any) {
                vscode.window.showErrorMessage(`Failed to download glslang: ${err?.message || err}`);
            }
        })
    );

    await startLanguageServer(context, outputChannel);
}

export async function deactivate(): Promise<void> {
    if (client) {
        await client.stop();
        client = undefined;
    }
}

async function startLanguageServer(context: vscode.ExtensionContext, outputChannel: vscode.OutputChannel) {
    try {
        const validatorPath = await resolveGlslValidator(context, outputChannel);
        if (!validatorPath) {
            outputChannel.appendLine('[GLSL Extended] Language server executable not available. LSP disabled.');
            return;
        }

        const glslangPath = await resolveGlslang(context, outputChannel);
        const config = vscode.workspace.getConfiguration('glslExtended');
        const analyzerPath = config.get<string>('analyzerPath') || undefined;

        outputChannel.appendLine(`[GLSL Extended] Using glsl_validator: ${validatorPath}`);
        if (glslangPath) {
            outputChannel.appendLine(`[GLSL Extended] Using glslangValidator: ${glslangPath}`);
        }

        const env: Record<string, string> = { ...process.env as Record<string, string> };
        if (glslangPath) {
            env['GLSLANG_VALIDATOR_PATH'] = glslangPath;
        }
        if (analyzerPath) {
            env['GLSL_ANALYZER_PATH'] = analyzerPath;
        }

        const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
        const serverOptions: ServerOptions = {
            run: {
                command: validatorPath,
                args: [],
                options: { env, cwd }
            },
            debug: {
                command: validatorPath,
                args: [],
                options: { env, cwd }
            }
        };

        const clientOptions: LanguageClientOptions = {
            documentSelector: [{ scheme: 'file', language: 'glsl' }],
            synchronize: {
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{vert,frag,geom,comp,tesc,tese,mesh,task,rgen,rint,rahit,rchit,rmiss,rcall,glsl,glslh}')
            },
            initializationOptions: {
                glslang_validator_path: glslangPath,
                glsl_analyzer_path: analyzerPath
            },
            outputChannel
        };

        client = new LanguageClient(
            'glsl_validator',
            'GLSL Validator Language Server',
            serverOptions,
            clientOptions
        );

        const traceSetting = config.get<string>('trace.server', 'off');
        if (traceSetting === 'verbose') {
            client.setTrace(Trace.Verbose);
        } else if (traceSetting === 'messages') {
            client.setTrace(Trace.Messages);
        }

        await client.start();
        outputChannel.appendLine('[GLSL Extended] Language Server successfully started.');
    } catch (err: any) {
        outputChannel.appendLine(`[GLSL Extended] Failed to start Language Server: ${err?.message || err}`);
        vscode.window.showErrorMessage(`GLSL Extended LSP failed to start: ${err?.message || err}`);
    }
}

async function resolveGlslValidator(
    context: vscode.ExtensionContext,
    outputChannel: vscode.OutputChannel
): Promise<string | undefined> {
    const exeName = process.platform === 'win32' ? 'glsl_validator.exe' : 'glsl_validator';

    // 0. Check bundled binary inside extension folder
    const bundledBin = path.join(context.extensionPath, 'bin', exeName);
    if (fs.existsSync(bundledBin) && fs.statSync(bundledBin).isFile()) {
        outputChannel.appendLine(`[GLSL Extended] Using bundled binary: ${bundledBin}`);
        return bundledBin;
    }

    // 1. Check user explicit setting
    const config = vscode.workspace.getConfiguration('glslExtended');
    const configured = config.get<string>('validatorPath')?.trim();
    if (configured && fs.existsSync(configured) && fs.statSync(configured).isFile()) {
        return configured;
    }

    // 2. Check PATH
    const inPath = findInPath(exeName);
    if (inPath) {
        return inPath;
    }

    // 3. Check local extension storage
    const storageDir = context.globalStorageUri.fsPath;
    const cachedBin = path.join(storageDir, 'bin', exeName);
    const cachedRoot = path.join(storageDir, exeName);

    if (fs.existsSync(cachedBin) && fs.statSync(cachedBin).isFile()) {
        return cachedBin;
    }
    if (fs.existsSync(cachedRoot) && fs.statSync(cachedRoot).isFile()) {
        return cachedRoot;
    }

    // 4. Download from GitHub Releases
    return await downloadGlslValidator(context, outputChannel, false);
}

function findFileRecursive(dir: string, targetName: string): string | undefined {
    if (!fs.existsSync(dir)) return undefined;
    try {
        const entries = fs.readdirSync(dir, { withFileTypes: true });
        for (const entry of entries) {
            const fullPath = path.join(dir, entry.name);
            if (entry.isFile()) {
                if (entry.name.toLowerCase() === targetName.toLowerCase()) {
                    return fullPath;
                }
            } else if (entry.isDirectory()) {
                const found = findFileRecursive(fullPath, targetName);
                if (found) return found;
            }
        }
    } catch {
        // ignore inaccessible directories
    }
    return undefined;
}

async function resolveGlslang(
    context: vscode.ExtensionContext,
    outputChannel: vscode.OutputChannel
): Promise<string | undefined> {
    // 1. User setting
    const config = vscode.workspace.getConfiguration('glslExtended');
    const configured = config.get<string>('glslangValidatorPath')?.trim();
    if (configured && fs.existsSync(configured) && fs.statSync(configured).isFile()) {
        return configured;
    }

    // 2. PATH
    const exe = process.platform === 'win32' ? '.exe' : '';
    const inPath = findInPath(`glslangValidator${exe}`) || findInPath(`glslang${exe}`);
    if (inPath) {
        return inPath;
    }

    // 3. VULKAN_SDK
    const vkSdk = process.env['VULKAN_SDK'];
    if (vkSdk) {
        const c1 = path.join(vkSdk, 'bin', `glslangValidator${exe}`);
        const c2 = path.join(vkSdk, 'bin', `glslang${exe}`);
        if (fs.existsSync(c1)) return c1;
        if (fs.existsSync(c2)) return c2;
    }

    // 4. Platform common locations
    if (process.platform === 'win32') {
        const fallbacks = [
            'C:\\msys64\\ucrt64\\bin\\glslangValidator.exe',
            'C:\\msys64\\ucrt64\\bin\\glslang.exe',
            'C:\\msys64\\mingw64\\bin\\glslangValidator.exe',
            'C:\\msys64\\mingw64\\bin\\glslang.exe',
            'C:\\msys64\\clang64\\bin\\glslangValidator.exe',
            'C:\\msys64\\clang64\\bin\\glslang.exe',
            'C:\\Program Files\\glslang\\bin\\glslangValidator.exe',
            'C:\\Program Files\\glslang\\bin\\glslang.exe'
        ];
        for (const fb of fallbacks) {
            if (fs.existsSync(fb)) return fb;
        }
    } else {
        const fallbacks = [
            '/usr/local/bin/glslangValidator',
            '/usr/bin/glslangValidator',
            '/opt/homebrew/bin/glslangValidator',
            '/usr/local/bin/glslang',
            '/usr/bin/glslang',
            '/opt/homebrew/bin/glslang'
        ];
        for (const fb of fallbacks) {
            if (fs.existsSync(fb)) return fb;
        }
    }

    // 5. Check local extension storage (cached download)
    const storageDir = context.globalStorageUri.fsPath;
    const glslangDir = path.join(storageDir, 'glslang');
    const exeName = `glslangValidator${exe}`;
    const altExeName = `glslang${exe}`;

    const cached1 = findFileRecursive(glslangDir, exeName) || findFileRecursive(glslangDir, altExeName);
    if (cached1 && fs.existsSync(cached1) && fs.statSync(cached1).isFile()) {
        return cached1;
    }
    const cached2 = findFileRecursive(storageDir, exeName) || findFileRecursive(storageDir, altExeName);
    if (cached2 && fs.existsSync(cached2) && fs.statSync(cached2).isFile()) {
        return cached2;
    }

    // 6. Download automatically from KhronosGroup/glslang releases
    try {
        return await downloadGlslang(context, outputChannel);
    } catch (err: any) {
        outputChannel.appendLine(`[GLSL Extended] Auto-download of glslang failed: ${err?.message || err}`);
        return undefined;
    }
}

async function downloadGlslang(
    context: vscode.ExtensionContext,
    outputChannel: vscode.OutputChannel
): Promise<string> {
    const storageDir = context.globalStorageUri.fsPath;
    const glslangDir = path.join(storageDir, 'glslang');
    if (!fs.existsSync(glslangDir)) {
        fs.mkdirSync(glslangDir, { recursive: true });
    }

    const platform = process.platform;
    const arch = process.arch;

    return await vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'GLSL Extended: Downloading glslang reference compiler',
            cancellable: false
        },
        async (progress) => {
            progress.report({ message: 'Checking KhronosGroup/glslang releases...' });
            outputChannel.appendLine('[GLSL Extended] Fetching latest glslang release from GitHub...');

            const releaseUrl = 'https://api.github.com/repos/KhronosGroup/glslang/releases/latest';
            const releaseData = await httpGetJson(releaseUrl);

            const asset = releaseData.assets?.find((a: any) => {
                const name = (a.name || '').toLowerCase();
                if (!name.includes('release') || name.includes('debug')) {
                    return false;
                }
                if (platform === 'win32') {
                    return name.includes('windows-x86_64') || (arch === 'arm64' && name.includes('windows-arm64'));
                } else if (platform === 'linux') {
                    return name.includes('linux-x86_64') || (arch === 'arm64' && name.includes('linux-arm64'));
                } else if (platform === 'darwin') {
                    return name.includes('macos-universal') || name.includes('macos');
                }
                return false;
            });

            if (!asset || !asset.browser_download_url) {
                throw new Error(`No compatible glslang release asset found for ${platform}-${arch} in release ${releaseData.tag_name || 'latest'}`);
            }

            const downloadUrl = asset.browser_download_url;
            const archivePath = path.join(storageDir, asset.name);

            progress.report({ message: `Downloading ${asset.name}...` });
            outputChannel.appendLine(`[GLSL Extended] Downloading ${downloadUrl} to ${archivePath}`);
            await downloadFile(downloadUrl, archivePath);

            progress.report({ message: 'Extracting glslang compiler...' });
            outputChannel.appendLine(`[GLSL Extended] Extracting ${archivePath} to ${glslangDir}`);
            extractArchive(archivePath, glslangDir);

            // Clean up archive
            try { fs.unlinkSync(archivePath); } catch {}

            const exe = platform === 'win32' ? '.exe' : '';
            const exeName = `glslangValidator${exe}`;
            const altExeName = `glslang${exe}`;

            const targetPath = findFileRecursive(glslangDir, exeName) || findFileRecursive(glslangDir, altExeName);
            if (!targetPath) {
                throw new Error(`Extracted glslang executable not found in ${glslangDir}`);
            }

            if (platform !== 'win32') {
                fs.chmodSync(targetPath, 0o755);
            }

            outputChannel.appendLine(`[GLSL Extended] glslang installed successfully at ${targetPath}`);
            return targetPath;
        }
    );
}

function findInPath(binName: string): string | undefined {
    const pathEnv = process.env.PATH || '';
    const delimiter = process.platform === 'win32' ? ';' : ':';
    const dirs = pathEnv.split(delimiter);

    for (const dir of dirs) {
        if (!dir.trim()) continue;
        const full = path.join(dir.trim(), binName);
        try {
            if (fs.existsSync(full) && fs.statSync(full).isFile()) {
                return full;
            }
        } catch {
            // ignore inaccessible directories
        }
    }
    return undefined;
}

async function downloadGlslValidator(
    context: vscode.ExtensionContext,
    outputChannel: vscode.OutputChannel,
    force: boolean
): Promise<string> {
    const storageDir = context.globalStorageUri.fsPath;
    if (!fs.existsSync(storageDir)) {
        fs.mkdirSync(storageDir, { recursive: true });
    }

    const platform = process.platform;
    const arch = process.arch;

    let assetName = '';
    if (platform === 'win32') {
        assetName = 'glsl_validator-x86_64-windows.zip';
    } else if (platform === 'linux') {
        assetName = 'glsl_validator-x86_64-linux.tar.gz';
    } else if (platform === 'darwin') {
        assetName = arch === 'arm64' ? 'glsl_validator-aarch64-macos.tar.gz' : 'glsl_validator-x86_64-macos.tar.gz';
    } else {
        throw new Error(`Unsupported platform: ${platform} (${arch})`);
    }

    return await vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'GLSL Extended: Downloading Language Server',
            cancellable: false
        },
        async (progress) => {
            progress.report({ message: 'Checking GitHub releases...' });

            const releaseUrl = 'https://api.github.com/repos/zyr1on/glsl-extended/releases/latest';
            const releaseData = await httpGetJson(releaseUrl);

            const asset = releaseData.assets?.find((a: any) => a.name === assetName);
            if (!asset || !asset.browser_download_url) {
                throw new Error(`Release asset '${assetName}' not found in release ${releaseData.tag_name || 'latest'}`);
            }

            const downloadUrl = asset.browser_download_url;
            const archivePath = path.join(storageDir, assetName);

            progress.report({ message: `Downloading ${assetName}...` });
            outputChannel.appendLine(`[GLSL Extended] Downloading ${downloadUrl} to ${archivePath}`);
            await downloadFile(downloadUrl, archivePath);

            progress.report({ message: 'Extracting language server binary...' });
            outputChannel.appendLine(`[GLSL Extended] Extracting ${archivePath}`);
            extractArchive(archivePath, storageDir);

            // Clean up archive
            try { fs.unlinkSync(archivePath); } catch {}

            const exeName = platform === 'win32' ? 'glsl_validator.exe' : 'glsl_validator';
            const candidateRoot = path.join(storageDir, exeName);
            const candidateBin = path.join(storageDir, 'bin', exeName);

            let targetPath = '';
            if (fs.existsSync(candidateRoot)) {
                targetPath = candidateRoot;
            } else if (fs.existsSync(candidateBin)) {
                targetPath = candidateBin;
            } else {
                throw new Error(`Extracted executable not found in ${storageDir}`);
            }

            if (platform !== 'win32') {
                fs.chmodSync(targetPath, 0o755);
            }

            outputChannel.appendLine(`[GLSL Extended] Binary successfully installed at ${targetPath}`);
            return targetPath;
        }
    );
}

function httpGetJson(url: string): Promise<any> {
    return new Promise((resolve, reject) => {
        const options = {
            headers: {
                'User-Agent': 'vscode-glsl-extended'
            }
        };
        https.get(url, options, (res) => {
            if (res.statusCode && res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
                return httpGetJson(res.headers.location).then(resolve, reject);
            }
            if (res.statusCode !== 200) {
                return reject(new Error(`HTTP request failed with status: ${res.statusCode}`));
            }
            let data = '';
            res.on('data', (chunk) => data += chunk);
            res.on('end', () => {
                try {
                    resolve(JSON.parse(data));
                } catch (e) {
                    reject(e);
                }
            });
        }).on('error', reject);
    });
}

function downloadFile(url: string, destPath: string): Promise<void> {
    return new Promise((resolve, reject) => {
        const fileStream = fs.createWriteStream(destPath);
        const options = {
            headers: {
                'User-Agent': 'vscode-glsl-extended'
            }
        };
        const request = https.get(url, options, (res) => {
            if (res.statusCode && res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
                fileStream.close();
                return downloadFile(res.headers.location, destPath).then(resolve, reject);
            }
            if (res.statusCode !== 200) {
                fileStream.close();
                return reject(new Error(`Failed to download: HTTP ${res.statusCode}`));
            }
            res.pipe(fileStream);
            fileStream.on('finish', () => {
                fileStream.close();
                resolve();
            });
        });
        request.on('error', (err) => {
            fileStream.close();
            try { fs.unlinkSync(destPath); } catch {}
            reject(err);
        });
    });
}

function extractArchive(archivePath: string, destDir: string) {
    if (process.platform === 'win32') {
        if (archivePath.endsWith('.zip')) {
            execSync(`powershell -NoProfile -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${destDir}' -Force"`);
        } else {
            execSync(`tar -xf "${archivePath}" -C "${destDir}"`);
        }
    } else {
        if (archivePath.endsWith('.tar.gz') || archivePath.endsWith('.tgz')) {
            execSync(`tar -xzf "${archivePath}" -C "${destDir}"`);
        } else if (archivePath.endsWith('.zip')) {
            execSync(`unzip -o "${archivePath}" -d "${destDir}"`);
        }
    }
}
