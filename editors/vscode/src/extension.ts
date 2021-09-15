import * as client from "./client";
import { ExtensionContext, commands, window } from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

let languageClient: LanguageClient | undefined;

const DDLOG_PROJECT_CONTEXT_NAME = "inDDlogProject";

export async function activate(context: ExtensionContext) {
    await tryActivate(context).catch((error) => {
        window.showErrorMessage(`Cannot activate ddlog-lsp: ${error.message}`);
        throw error;
    });
}

async function tryActivate(context: ExtensionContext) {
    const result = await client.launch(context);
    languageClient = result;

    await setContextValue(DDLOG_PROJECT_CONTEXT_NAME, true);
}

export async function deactivate() {
    await setContextValue(DDLOG_PROJECT_CONTEXT_NAME, undefined);

    if (languageClient) {
        await languageClient.stop();
    }
}

/**
 * Sets ['when'](https://code.visualstudio.com/docs/getstarted/keybindings#_when-clause-contexts) clause contexts
 */
function setContextValue(key: string, value: any): Thenable<void> {
    return commands.executeCommand("setContext", key, value);
}

// TODO: Problem matchers, commands, settings, etc.
// TODO: Bundle the extension https://aka.ms/vscode-bundle-extension
// TODO: Write a .vscodeignore https://aka.ms/vscode-bundle-extension
