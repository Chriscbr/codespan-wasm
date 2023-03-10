"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.emit = void 0;
async function emit(code, config) {
    return Promise.resolve().then(() => require("../pkg")).then((mod) => {
        return mod.emit(code, config);
    });
}
exports.emit = emit;
emit("let x = 1 + 1;", {
    displayStyle: "rich",
}).then((result) => {
    console.log(result);
}).catch((err) => {
    console.error(err);
});
