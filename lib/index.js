"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.emit = exports.ASCII_CHARS = exports.BOX_DRAWING_CHARS = void 0;
exports.BOX_DRAWING_CHARS = {
    snippetStart: "┌─",
    sourceBorderLeft: '│',
    sourceBorderLeftBreak: '·',
    noteBullet: '=',
    singlePrimaryCaret: '^',
    singleSecondaryCaret: '-',
    multiPrimaryCaretStart: '^',
    multiPrimaryCaretEnd: '^',
    multiSecondaryCaretStart: '\'',
    multiSecondaryCaretEnd: '\'',
    multiTopLeft: '╭',
    multiTop: '─',
    multiBottomLeft: '╰',
    multiBottom: '─',
    multiLeft: '│',
    pointerLeft: '│',
};
exports.ASCII_CHARS = {
    snippetStart: "-->",
    sourceBorderLeft: '|',
    sourceBorderLeftBreak: '.',
    noteBullet: '=',
    singlePrimaryCaret: '^',
    singleSecondaryCaret: '-',
    multiPrimaryCaretStart: '^',
    multiPrimaryCaretEnd: '^',
    multiSecondaryCaretStart: '\'',
    multiSecondaryCaretEnd: '\'',
    multiTopLeft: '/',
    multiTop: '-',
    multiBottomLeft: '\\',
    multiBottom: '-',
    multiLeft: '|',
    pointerLeft: '|',
};
const codespan_bindings = Promise.resolve().then(() => require("../pkg"));
async function emit(files, diagnostic, config) {
    return codespan_bindings.then((mod) => {
        return mod.emit(files, diagnostic, config);
    });
}
exports.emit = emit;
