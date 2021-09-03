import { ExtensionContext, workspace } from "vscode";
import {
    LanguageClient,
    ServerOptions,
    Middleware,
    LanguageClientOptions,
    StreamInfo,
} from "vscode-languageclient";
import * as net from "net";

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
    /*
    const run: Executable = {
        // FIXME: Change exe name & args
        command: "ddlog-driver",
    };
    const debug: Executable = {
        // FIXME: Change exe name & args
        command: "ddlog-driver",
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
    */
    let serverOptions = createSocketServer(context);

    const clientOptions: LanguageClientOptions = {
        diagnosticCollectionName: "ddlog-lsp",
        documentSelector: [
            { language: "ddlog.dl", scheme: "file" },
            { language: "ddlog.dat", scheme: "file" },
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
