interface Window {
  __TAURI__: {
    event: {
      listen: (event: EventName, callback: EventCallback<T>) => Promise<UnlistenFn>
    }
    tauri: {
      invoke:(cmd: string, args: InvokeArgs = {}) => Promise<T>
    }
    dialog: {
      open:(options?: OpenDialogOptions)=> Promise<null | string | string[]>,
      message: (message: string, options?: string | MessageDialogOptions)=> Promise<void>
    }
  }
}
type EventName = string;
type EventCallback<T> = (event: Event<T>) => void;
type UnlistenFn = () => void;
type InvokeArgs = Record<string, unknown>
interface Event<T> {
  /** Event name */
  event: EventName
  /** The label of the window that emitted this event. */
  windowLabel: string
  /** Event identifier used to unlisten */
  id: number
  /** Event payload */
  payload: T
}
// dialogs

interface OpenDialogOptions {
  /** The title of the dialog window. */
  title?: string
  /** The filters of the dialog. */
  filters?: DialogFilter[]
  /** Initial directory or file path. */
  defaultPath?: string
  /** Whether the dialog allows multiple selection or not. */
  multiple?: boolean
  /** Whether the dialog is a directory selection or not. */
  directory?: boolean
  /**
   * If `directory` is true, indicates that it will be read recursively later.
   * Defines whether subdirectories will be allowed on the scope or not.
   */
  recursive?: boolean
}

interface MessageDialogOptions {
  /** The title of the dialog. Defaults to the app name. */
  title?: string
  /** The type of the dialog. Defaults to `info`. */
  type?: 'info' | 'warning' | 'error'
  /** The label of the confirm button. */
  okLabel?: string
}