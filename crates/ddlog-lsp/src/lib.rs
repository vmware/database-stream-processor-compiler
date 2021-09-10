mod backend;
mod database;
mod providers;
mod session;

pub use backend::Backend;
pub use session::Session;

// TODO: Once we have an interpreter and stuff running, the vscode notebook
//       api could be amazing for debugging, learning and just messing around
//       https://vscode-westeu.azurewebsites.net/api/extension-guides/notebook
