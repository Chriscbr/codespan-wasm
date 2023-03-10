"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.emit = exports.CHARS_ASCII = exports.CHARS_BOX_DRAWING = void 0;
exports.CHARS_BOX_DRAWING = {
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
exports.CHARS_ASCII = {
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
