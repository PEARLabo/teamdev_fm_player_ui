const { open, message } = window.__TAURI__.dialog;

export async function errorDialog(msg:string, title = "Error") {
    console.error(msg);
    message(msg, { title, type: "error" });
}

export async function warningDialog(msg:string, title = "Warning") {
    console.warn(msg);
    message(msg, { title, type: "warning" });
}

export async function infoDialog(msg:string, title = "Info") {
    console.log(msg);
    message(msg, { title, type: "info" });
}

export { open };
