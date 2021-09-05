import * as net from "net";
import { ExtensionContext, workspace } from "vscode";
import { LanguageClientOptions, Middleware } from "vscode-languageclient";
import {
    ServerOptions,
    StreamInfo,
    LanguageClient,
} from "vscode-languageclient/node";

function createSocketServer(_context: ExtensionContext): ServerOptions {
    // The server is a started as a separate app and listens on port 5007
    let connectionInfo: net.TcpNetConnectOpts = {
        port: 5007,
    };

    let serverOptions = () => {
        // Connect to language server via socket
        let socket = net.connect(connectionInfo);
        let result: StreamInfo = {
            writer: socket,
            reader: socket,
        };

        return Promise.resolve(result);
    };

    return serverOptions;
}

export async function launch(
    context: ExtensionContext
): Promise<LanguageClient> {
    let serverOptions = createSocketServer(context);

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
