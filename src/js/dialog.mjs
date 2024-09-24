const { open, message } = window.__TAURI__.dialog;
/**
 *
 * @param {String} msg
 * @param {Promise<void>} title
 */
export async function errorDialog(msg, title = "Error") {
    console.error(msg);
    message(msg, { title, type: "error" });
}
/**
 *
 * @param {String} msg
 * @param {Promise<void>} title
 */
export async function warningDialog(msg, title = "Warning") {
    console.warn(msg);
    message(msg, { title, type: "warning" });
}
/**
 *
 * @param {String} msg
 * @param {Promise<void>} title
 */
export async function infoDialog(msg, title = "Info") {
    console.log(msg);
    message(msg, { title, type: "info" });
}

export { open };
