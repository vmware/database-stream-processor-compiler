import * as client from "./client";
import { ExtensionContext } from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

let languageClient: LanguageClient | undefined;

export async function activate(context: ExtensionContext) {
    const result = await client.launch(context);
    languageClient = result;
}

export async function deactivate() {
    if (languageClient) {
        await languageClient.stop();
    }
}

// TODO: Problem matchers, commands, settings, etc.
// TODO: Bundle the extension https://aka.ms/vscode-bundle-extension
// TODO: Write a .vscodeignore https://aka.ms/vscode-bundle-extension
