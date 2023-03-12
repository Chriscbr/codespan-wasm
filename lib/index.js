"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.emitDiagnostic = exports.CHARS_ASCII = exports.CHARS_BOX_DRAWING = void 0;
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
/**
 * Emit a diagnostic using the given files, context, and config.
 */
async function emitDiagnostic(files, diagnostic, config, coloring) {
    return codespan_bindings.then((mod) => {
        return mod.emit_diagnostic(files, diagnostic, config, coloring);
    });
}
exports.emitDiagnostic = emitDiagnostic;
