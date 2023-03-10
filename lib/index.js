"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.emit = void 0;
const codespan_bindings = Promise.resolve().then(() => require("../pkg"));
async function emit(files, diagnostic, config) {
    return codespan_bindings.then((mod) => {
        return mod.emit(files, diagnostic, config);
    });
}
exports.emit = emit;
