"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.emit = void 0;
async function emit(code, diagnostic, config) {
    return Promise.resolve().then(() => require("../pkg")).then((mod) => {
        return mod.emit(code, diagnostic, config);
    });
}
exports.emit = emit;
emit("let x = 1 + 1;", {
    severity: "error",
    message: "`case` clauses have incompatible types",
    code: "E0308",
    labels: [
        {
            style: "primary",
            fileId: "0",
            rangeStart: 328,
            rangeEnd: 331,
            message: "expected `String`, found `Nat`",
        },
        {
            style: "secondary",
            fileId: "0",
            rangeStart: 211,
            rangeEnd: 331,
            message: "`case` clauses have incompatible types",
        },
        {
            style: "secondary",
            fileId: "0",
            rangeStart: 258,
            rangeEnd: 268,
            message: "this is found to be of type `String`",
        },
        {
            style: "secondary",
            fileId: "0",
            rangeStart: 284,
            rangeEnd: 290,
            message: "this is found to be of type `String`",
        },
        {
            style: "secondary",
            fileId: "0",
            rangeStart: 306,
            rangeEnd: 312,
            message: "this is found to be of type `String`",
        },
        {
            style: "secondary",
            fileId: "0",
            rangeStart: 186,
            rangeEnd: 182,
            message: "expected type `String` found here",
        },
    ],
    notes: [`expected type \`String\`
    found type \`Nat\``],
}, {
    displayStyle: "rich",
}).then((result) => {
    console.log(result);
}).catch((err) => {
    console.error(err);
});
