import * as client from "./client";
import { LanguageClient } from "vscode-languageclient";
import { ExtensionContext } from "vscode";

let languageClient: LanguageClient | undefined;

export async function activate(context: ExtensionContext) {
    const result = await client.launch(context);
    languageClient = result;
}

export async function deactivate() {
    if (languageClient === undefined || languageClient === null) {
        return;
    }

    await languageClient.stop();
}
