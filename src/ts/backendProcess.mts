const { invoke } = window.__TAURI__.tauri;
class TauriBackEnd {
    serialport:SerialPort;
    constructor() {
        this.serialport = new SerialPort();
        window.__TAURI__.event.listen("error", console.error);
    }
    set onerror(callback:EventCallback<unknown>) {
        window.__TAURI__.event.listen("error", callback);
    }
    set onmessage(callback:EventCallback<unknown>) {
        window.__TAURI__.event.listen("message", callback);
    }
    set onseq_msg(callback:EventCallback<SequencerInfo>) {
        window.__TAURI__.event.listen("sequencer-msg", callback);
    }
    file_open(path:string) {
        return invoke("open_file", { path });
    }
    send_file() {
        return invoke("send_midi_file");
    }
}
class SerialPort {
    /**
     * @description This function does not return success or failure.
     * @returns
     */
    open(portName:string) {
        return invoke("serialport_open", { portName });
    }
    /**
     * @description This function does not return success or failure.
     * @returns
     */
    close() {
        return invoke("serialport_close");
    }
    get_available_ports() {
        return invoke("get_available_serial_ports", {});
    }
}
export const BackEnd = new TauriBackEnd();
