/**
 * Mantra WebAssembly Browser Runtime
 * Provides browser-host interoperability, standard imports, memory access, and function execution.
 */
class MantraBrowserRuntime {
    constructor() {
        this.instance = null;
        this.memory = null;
        this.outputLogs = [];
    }

    /**
     * Initializes the WebAssembly module from a binary or buffer.
     * @param {string | ArrayBuffer} wasmSource 
     */
    async load(wasmSource) {
        const importObject = {
            env: {
                print: (val) => {
                    const msg = `[Mantra WASM Output] ${val}`;
                    console.log(msg);
                    this.outputLogs.push(String(val));
                    this.notifyListeners(String(val));
                },
                print_str: (ptr, len) => {
                    const text = this.readString(ptr, len);
                    console.log(`[Mantra WASM String] ${text}`);
                    this.outputLogs.push(text);
                    this.notifyListeners(text);
                },
                now: () => Date.now(),
            }
        };

        if (typeof wasmSource === "string") {
            const response = await fetch(wasmSource);
            const { instance } = await WebAssembly.instantiateStreaming(response, importObject);
            this.instance = instance;
        } else {
            const { instance } = await WebAssembly.instantiate(wasmSource, importObject);
            this.instance = instance;
        }

        this.memory = this.instance.exports.memory;
        console.log("Mantra WebAssembly Instance Ready with exports:", Object.keys(this.instance.exports));
        return this;
    }

    /**
     * Reads a UTF-8 string from the WebAssembly linear memory.
     */
    readString(ptr, len) {
        if (!this.memory) return "";
        const bytes = new Uint8Array(this.memory.buffer, Number(ptr), Number(len));
        return new TextDecoder("utf-8").decode(bytes);
    }

    /**
     * Invokes an exported Mantra WebAssembly function.
     */
    call(funcName, ...args) {
        if (!this.instance || !this.instance.exports[funcName]) {
            throw new Error(`Exported function '${funcName}' not found in Mantra WASM module.`);
        }
        // BigInt conversion for 64-bit integer arguments
        const normalizedArgs = args.map(arg => typeof arg === "number" ? BigInt(arg) : arg);
        const result = this.instance.exports[funcName](...normalizedArgs);
        return typeof result === "bigint" ? Number(result) : result;
    }

    onLog(callback) {
        this._listener = callback;
    }

    notifyListeners(msg) {
        if (this._listener) {
            this._listener(msg);
        }
    }
}

window.MantraBrowserRuntime = MantraBrowserRuntime;

