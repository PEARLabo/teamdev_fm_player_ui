/**
 * @description 周期タスクを発行するクラス。ブラウザのレンダリングに同期して
 */
export default class PeriodicTask {
    #initialize = [];
    #tasks = [];
    #animate_id;
    /**
     *
     * @param {()=>void | [()=>void] | undefined} tasks 定期的に実行するタスク
     * @param {()=>void  |  [()=>void]  | undefined} init 定期タスクを開始する前に実行する処理
     */
    constructor(tasks, init) {
        if (!tasks) return;
        if (!Array.isArray(tasks)) {
            this.#tasks.push(tasks);
        } else {
            for (const task of tasks) {
                this.#tasks.push(task);
            }
        }
        if (!init) return;
        if (!Array.isArray(init)) {
            this.#initialize.push(init);
        } else {
            for (const init of init) {
                this.#initialize.push(init);
            }
        }
    }
    push(task, init) {
        this.#tasks.push(task);
        if (init) {
            this.#initialize.push(init);
        }
    }
    #execute() {
        for (const task of this.#tasks) {
            task();
        }
        this.#animate_id = requestAnimationFrame(this.#execute.bind(this));
    }
    start() {
        if (!this.#animate_id) {
            for (const init of this.#initialize) {
                init();
            }
            this.#animate_id = requestAnimationFrame(this.#execute.bind(this));
        } else {
            console.warn("Animation is already started");
        }
    }
    stop() {
        if (this.#animate_id) {
            cancelAnimationFrame(this.#animate_id);
            this.#animate_id = 0;
        } else {
            console.warn("Animation is not started");
        }
    }
}
