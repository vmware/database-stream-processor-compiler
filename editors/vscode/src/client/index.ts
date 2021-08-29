import { ExtensionContext, workspace } from "vscode";
import {
    LanguageClient,
    Executable,
    ServerOptions,
    Middleware,
    LanguageClientOptions,
} from "vscode-languageclient";

export async function launch(
    context: ExtensionContext
): Promise<LanguageClient> {
    const run: Executable = {
        command: "ddlog-lsp",
    };
    const debug: Executable = {
        command: "ddlog-lsp",
        options: {
            env: {
                RUST_BACKTRACE: 1,
                // TODO: activate ddlog's logger (probably with a cli argument)
                // RUST_LOG: "info",
                ...process.env,
            },
        },
    };

    const serverOptions: ServerOptions = { debug, run };
    const clientOptions: LanguageClientOptions = {
        diagnosticCollectionName: "ddlog-lsp",
        documentSelector: [
            { language: "ddlog", scheme: "file" },
            { language: "ddlog", scheme: "untitled" },
            { language: "ddlog-command", scheme: "file" },
            { language: "ddlog-command", scheme: "untitled" },
        ],
        synchronize: {
            fileEvents: [
                workspace.createFileSystemWatcher("**/*.dl"),
                workspace.createFileSystemWatcher("**/*.dat"),
            ],
        },
        middleware: {} as Middleware,
    };

    const languageClient = new LanguageClient(
        "ddlog-lsp",
        "DDlog Language Server",
        serverOptions,
        clientOptions
    );

    const session = languageClient.start();
    context.subscriptions.push(session);

    await languageClient.onReady();
    return languageClient;
}
