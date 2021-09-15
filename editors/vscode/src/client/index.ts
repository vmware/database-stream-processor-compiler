import * as net from "net";
import {
    // CancellationToken,
    // commands,
    // Event,
    // EventEmitter,
    ExtensionContext,
    // ProviderResult,
    // Range,
    // TextDocument,
    // TextDocumentChangeEvent,
    // TextDocumentContentProvider,
    // TextEditor,
    // Uri,
    // window,
    workspace,
} from "vscode";
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
        // Note: At one point I attempted to do `ddlog.dl` and `ddlog.dat` as the
        //       respective language ids, but VSCode has a bug in it somewhere
        //       that causes incredibly strange behavior with language ids that
        //       contain `.` within them
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
                // TODO: Once we have a ddlog config file we want to watch that as well,
                //       and we probably want to provide completions and/or schemas for it
                //       as well
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
    // context.subscriptions.push(
    //     commands.registerCommand(
    //         "ddlog-lsp.syntaxTree",
    //         viewSyntaxTree,
    //         languageClient
    //     )
    // );

    await languageClient.onReady();
    return languageClient;
}

/*
function viewSyntaxTree(this: LanguageClient) {
    const editor = window.activeTextEditor;
    if (!editor) {
        return;
    }

    let uri = editor.document.uri;
    let selection = editor.selection;
    const languageId = editor.document.languageId;

    // Ensure that the current document is a ddlog one
    if (languageId !== "ddlog" && languageId !== "ddlog-command") {
        return;
    }

    // Get the selected range within the current document, if it exists
    let range = editor.selection.isEmpty
        ? null
        : new Range(selection.start, selection.end);

    // Make the params we're gonna send to the backend
    let params = { textDocument: { uri: uri.toString(), languageId }, range };

    this.info(`${uri}, ${languageId}, ${range}`);
}

class SyntaxTreeDocumentProvider implements TextDocumentContentProvider {
    client: LanguageClient;

    readonly uri = Uri.parse("ddlog-lsp://syntax_tree/tree.rast");
    readonly eventEmitter = new EventEmitter<Uri>();

    constructor(client: LanguageClient) {
        this.client = client;

        workspace.onDidChangeTextDocument(this.onDidChangeTextDocument, this);
        window.onDidChangeActiveTextEditor(
            this.onDidChangeActiveTextEditor,
            this
        );
    }

    private onDidChangeTextDocument(event: TextDocumentChangeEvent) {
        if (isDDlogDocument(event.document)) {
            // We need to order this after language server updates, but there's no API for that.
            // Hence, good old sleep().
            void sleep(10).then(() => this.eventEmitter.fire(this.uri));
        }
    }

    private onDidChangeActiveTextEditor(editor: TextEditor | undefined) {
        if (editor && isDDlogEditor(editor)) {
            this.eventEmitter.fire(this.uri);
        }
    }

    provideTextDocumentContent(
        uri: Uri,
        ct: CancellationToken
    ): ProviderResult<string> {
        const params = {
            textDocument: { uri: rustEditor.document.uri.toString() },
            range,
        };

        return this.client.sendRequest("ddlog-lsp/syntaxTree", params, ct);
    }

    get onDidChange(): Event<Uri> {
        return this.eventEmitter.event;
    }
}

export type DDlogDocument = TextDocument & {
    languageId: "ddlog" | "ddlog-command";
};
export type DDlogEditor = TextEditor & { document: DDlogDocument };

export function isDDlogDocument(
    document: TextDocument
): document is DDlogDocument {
    // Prevent corrupted text (particularly via inlay hints) in diff views
    // by allowing only `file` schemes
    // unfortunately extensions that use diff views not always set this
    // to something different than 'file'
    return (
        (document.languageId === "ddlog" ||
            document.languageId === "ddlog-command") &&
        document.uri.scheme === "file"
    );
}

export function isDDlogEditor(editor: TextEditor): editor is DDlogEditor {
    return isDDlogDocument(editor.document);
}

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
*/
