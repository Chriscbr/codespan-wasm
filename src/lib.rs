use std::collections::HashMap;

use codespan_reporting::term;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use codespan_reporting::diagnostic::{Diagnostic as CodespanDiagnostic, Label as CodespanLabel};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{
    BufferWriter, Color as TermColor, ColorChoice, ColorSpec as TermColorSpec,
};
use codespan_reporting::term::{
    Chars as CodespanChars, Config as CodespanConfig, DisplayStyle as CodespanDisplayStyle,
    Styles as CodespanStyles,
};

#[wasm_bindgen(inline_js = "exports.error = function(s) { throw new Error(s) }")]
extern "C" {
    fn error(s: &str);
}

#[wasm_bindgen(inline_js = "exports.debug = function(s) { require('debug')('codespan-wasm')(s) }")]
extern "C" {
    fn debug(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
enum DisplayStyle {
    #[serde(rename = "rich")]
    Rich,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "short")]
    Short,
}

impl From<DisplayStyle> for CodespanDisplayStyle {
    fn from(display_style: DisplayStyle) -> Self {
        match display_style {
            DisplayStyle::Rich => CodespanDisplayStyle::Rich,
            DisplayStyle::Medium => CodespanDisplayStyle::Medium,
            DisplayStyle::Short => CodespanDisplayStyle::Short,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "magenta")]
    Magenta,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "white")]
    White,
}

impl From<Color> for TermColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => TermColor::Black,
            Color::Blue => TermColor::Blue,
            Color::Green => TermColor::Green,
            Color::Red => TermColor::Red,
            Color::Cyan => TermColor::Cyan,
            Color::Magenta => TermColor::Magenta,
            Color::Yellow => TermColor::Yellow,
            Color::White => TermColor::White,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ColorSpec {
    #[serde(rename = "fgColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fg_color: Option<Color>,
    #[serde(rename = "bgColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    bg_color: Option<Color>,
    bold: bool,
    intense: bool,
    underline: bool,
    dimmed: bool,
    italic: bool,
    reset: bool,
}

impl From<ColorSpec> for TermColorSpec {
    fn from(color_spec: ColorSpec) -> Self {
        let mut term_color_spec = TermColorSpec::new();
        if let Some(fg_color) = color_spec.fg_color {
            term_color_spec.set_fg(Some(fg_color.into()));
        }
        if let Some(bg_color) = color_spec.bg_color {
            term_color_spec.set_bg(Some(bg_color.into()));
        }
        term_color_spec.set_bold(color_spec.bold);
        term_color_spec.set_intense(color_spec.intense);
        term_color_spec.set_underline(color_spec.underline);
        term_color_spec.set_dimmed(color_spec.dimmed);
        term_color_spec.set_italic(color_spec.italic);
        term_color_spec.set_reset(color_spec.reset);
        term_color_spec
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Styles {
    #[serde(rename = "headerBug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    header_bug: Option<ColorSpec>,
    #[serde(rename = "headerError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    header_error: Option<ColorSpec>,
    #[serde(rename = "headerWarning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    header_warning: Option<ColorSpec>,
    #[serde(rename = "headerNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    header_note: Option<ColorSpec>,
    #[serde(rename = "headerHelp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    header_help: Option<ColorSpec>,
    #[serde(rename = "headerMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    header_message: Option<ColorSpec>,
    #[serde(rename = "primaryLabelBug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_label_bug: Option<ColorSpec>,
    #[serde(rename = "primaryLabelError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_label_error: Option<ColorSpec>,
    #[serde(rename = "primaryLabelWarning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_label_warning: Option<ColorSpec>,
    #[serde(rename = "primaryLabelNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_label_note: Option<ColorSpec>,
    #[serde(rename = "primaryLabelHelp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_label_help: Option<ColorSpec>,
    #[serde(rename = "secondaryLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_label: Option<ColorSpec>,
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    line_number: Option<ColorSpec>,
    #[serde(rename = "sourceBorder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_border: Option<ColorSpec>,
    #[serde(rename = "noteBullet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    note_bullet: Option<ColorSpec>,
}

impl From<Styles> for CodespanStyles {
    fn from(styles: Styles) -> Self {
        let mut codespan_styles = CodespanStyles::default();
        if let Some(header_bug) = styles.header_bug {
            codespan_styles.header_bug = header_bug.into();
        }
        if let Some(header_error) = styles.header_error {
            codespan_styles.header_error = header_error.into();
        }
        if let Some(header_warning) = styles.header_warning {
            codespan_styles.header_warning = header_warning.into();
        }
        if let Some(header_note) = styles.header_note {
            codespan_styles.header_note = header_note.into();
        }
        if let Some(header_help) = styles.header_help {
            codespan_styles.header_help = header_help.into();
        }
        if let Some(header_message) = styles.header_message {
            codespan_styles.header_message = header_message.into();
        }
        if let Some(primary_label_bug) = styles.primary_label_bug {
            codespan_styles.primary_label_bug = primary_label_bug.into();
        }
        if let Some(primary_label_error) = styles.primary_label_error {
            codespan_styles.primary_label_error = primary_label_error.into();
        }
        if let Some(primary_label_warning) = styles.primary_label_warning {
            codespan_styles.primary_label_warning = primary_label_warning.into();
        }
        if let Some(primary_label_note) = styles.primary_label_note {
            codespan_styles.primary_label_note = primary_label_note.into();
        }
        if let Some(primary_label_help) = styles.primary_label_help {
            codespan_styles.primary_label_help = primary_label_help.into();
        }
        if let Some(secondary_label) = styles.secondary_label {
            codespan_styles.secondary_label = secondary_label.into();
        }
        if let Some(line_number) = styles.line_number {
            codespan_styles.line_number = line_number.into();
        }
        if let Some(source_border) = styles.source_border {
            codespan_styles.source_border = source_border.into();
        }
        if let Some(note_bullet) = styles.note_bullet {
            codespan_styles.note_bullet = note_bullet.into();
        }
        codespan_styles
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Chars {
    #[serde(rename = "snippetStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    snippet_start: Option<String>,
    #[serde(rename = "snippetBorderLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_border_left: Option<char>,
    #[serde(rename = "snippetBorderLeftBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_border_left_break: Option<char>,
    #[serde(rename = "noteBullet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    note_bullet: Option<char>,
    #[serde(rename = "singlePrimaryCaret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    single_primary_caret: Option<char>,
    #[serde(rename = "singleSecondaryCaret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    single_secondary_caret: Option<char>,
    #[serde(rename = "multiPrimaryCaretStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_primary_caret_start: Option<char>,
    #[serde(rename = "multiPrimaryCaretEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_primary_caret_end: Option<char>,
    #[serde(rename = "multiSecondaryCaretStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_secondary_caret_start: Option<char>,
    #[serde(rename = "multiSecondaryCaretEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_secondary_caret_end: Option<char>,
    #[serde(rename = "multiTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_top_left: Option<char>,
    #[serde(rename = "multiTop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_top: Option<char>,
    #[serde(rename = "multiBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_bottom_left: Option<char>,
    #[serde(rename = "multiBottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_bottom: Option<char>,
    #[serde(rename = "multiLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_left: Option<char>,
    #[serde(rename = "pointerLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pointer_left: Option<char>,
}

impl From<Chars> for CodespanChars {
    fn from(chars: Chars) -> Self {
        let mut codespan_chars = CodespanChars::default();
        if let Some(snippet_start) = chars.snippet_start {
            codespan_chars.snippet_start = snippet_start;
        }
        if let Some(source_border_left) = chars.source_border_left {
            codespan_chars.source_border_left = source_border_left;
        }
        if let Some(source_border_left_break) = chars.source_border_left_break {
            codespan_chars.source_border_left_break = source_border_left_break;
        }
        if let Some(note_bullet) = chars.note_bullet {
            codespan_chars.note_bullet = note_bullet;
        }
        if let Some(single_primary_caret) = chars.single_primary_caret {
            codespan_chars.single_primary_caret = single_primary_caret;
        }
        if let Some(single_secondary_caret) = chars.single_secondary_caret {
            codespan_chars.single_secondary_caret = single_secondary_caret;
        }
        if let Some(multi_primary_caret_start) = chars.multi_primary_caret_start {
            codespan_chars.multi_primary_caret_start = multi_primary_caret_start;
        }
        if let Some(multi_primary_caret_end) = chars.multi_primary_caret_end {
            codespan_chars.multi_primary_caret_end = multi_primary_caret_end;
        }
        if let Some(multi_secondary_caret_start) = chars.multi_secondary_caret_start {
            codespan_chars.multi_secondary_caret_start = multi_secondary_caret_start;
        }
        if let Some(multi_secondary_caret_end) = chars.multi_secondary_caret_end {
            codespan_chars.multi_secondary_caret_end = multi_secondary_caret_end;
        }
        if let Some(multi_top_left) = chars.multi_top_left {
            codespan_chars.multi_top_left = multi_top_left;
        }
        if let Some(multi_top) = chars.multi_top {
            codespan_chars.multi_top = multi_top;
        }
        if let Some(multi_bottom_left) = chars.multi_bottom_left {
            codespan_chars.multi_bottom_left = multi_bottom_left;
        }
        if let Some(multi_bottom) = chars.multi_bottom {
            codespan_chars.multi_bottom = multi_bottom;
        }
        if let Some(multi_left) = chars.multi_left {
            codespan_chars.multi_left = multi_left;
        }
        if let Some(pointer_left) = chars.pointer_left {
            codespan_chars.pointer_left = pointer_left;
        }
        codespan_chars
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(rename = "displayStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_style: Option<DisplayStyle>,
    #[serde(rename = "tabWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tab_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    styles: Option<Styles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chars: Option<Chars>,
    #[serde(rename = "startContextLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start_context_lines: Option<usize>,
    #[serde(rename = "endContextLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end_context_lines: Option<usize>,
}

impl From<Config> for CodespanConfig {
    fn from(config: Config) -> Self {
        let mut term_config = codespan_reporting::term::Config::default();
        if let Some(display_style) = config.display_style {
            term_config.display_style = display_style.into();
        }
        if let Some(tab_width) = config.tab_width {
            term_config.tab_width = tab_width;
        }
        if let Some(styles) = config.styles {
            term_config.styles = styles.into();
        }
        if let Some(chars) = config.chars {
            term_config.chars = chars.into();
        }
        if let Some(start_context_lines) = config.start_context_lines {
            term_config.start_context_lines = start_context_lines;
        }
        if let Some(end_context_lines) = config.end_context_lines {
            term_config.end_context_lines = end_context_lines;
        }
        term_config
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Severity {
    #[serde(rename = "bug")]
    Bug,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "help")]
    Help,
}

#[derive(Serialize, Deserialize, Debug)]
enum LabelStyle {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
}

#[derive(Serialize, Deserialize, Debug)]
struct Label {
    style: LabelStyle,
    #[serde(rename = "fileId")]
    file_id: String,
    #[serde(rename = "rangeStart")]
    range_start: usize,
    #[serde(rename = "rangeEnd")]
    range_end: usize,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Diagnostic {
    severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    message: String,
    #[serde(default = "Vec::new")]
    labels: Vec<Label>,
    #[serde(default = "Vec::new")]
    notes: Vec<String>,
}

fn convert_diagnostic(
    diagnostic: Diagnostic,
    handle_map: &HashMap<String, usize>,
) -> CodespanDiagnostic<usize> {
    let mut codespan_diagnostic = match diagnostic.severity {
        Severity::Bug => CodespanDiagnostic::bug(),
        Severity::Error => CodespanDiagnostic::error(),
        Severity::Warning => CodespanDiagnostic::warning(),
        Severity::Note => CodespanDiagnostic::note(),
        Severity::Help => CodespanDiagnostic::help(),
    };
    if let Some(code) = diagnostic.code {
        codespan_diagnostic = codespan_diagnostic.with_code(code);
    }
    codespan_diagnostic = codespan_diagnostic.with_message(diagnostic.message);
    codespan_diagnostic = codespan_diagnostic.with_labels(
        diagnostic
            .labels
            .into_iter()
            .filter_map(|label| {
                let file_id = match handle_map.get(&label.file_id) {
                    Some(file_id) => *file_id,
                    None => {
                        error(&format!(
                            "File ID \"{}\" is not one of the included files",
                            label.file_id
                        ));
                        return None;
                    }
                };
                let mut codespan_label = match label.style {
                    LabelStyle::Primary => {
                        CodespanLabel::primary(file_id, label.range_start..label.range_end)
                    }
                    LabelStyle::Secondary => {
                        CodespanLabel::secondary(file_id, label.range_start..label.range_end)
                    }
                };
                codespan_label = codespan_label.with_message(label.message);
                Some(codespan_label)
            })
            .collect(),
    );
    codespan_diagnostic = codespan_diagnostic.with_notes(diagnostic.notes);
    codespan_diagnostic
}

#[derive(Serialize, Deserialize, Debug)]
struct File {
    name: String,
    source: String,
}

#[wasm_bindgen]
pub fn emit(files: JsValue, diagnostic: JsValue, config: JsValue) -> String {
    let files: Vec<File> = match serde_wasm_bindgen::from_value(files) {
        Ok(files) => files,
        Err(err) => {
            error(&format!("{}", err));
            return String::from("Error");
        }
    };
    debug(&format!("files: {:?}", files));

    let mut file_db = SimpleFiles::new();
    let mut handle_map = HashMap::new();
    for file in files {
        let handle = file_db.add(file.name.clone(), file.source);
        handle_map.insert(file.name, handle);
    }

    let diagnostic: Diagnostic = match serde_wasm_bindgen::from_value(diagnostic) {
        Ok(diagnostic) => diagnostic,
        Err(err) => {
            error(&format!("{}", err));
            return String::from("Error");
        }
    };
    debug(&format!("diagnostic: {:?}", diagnostic));
    let diagnostic: CodespanDiagnostic<usize> = convert_diagnostic(diagnostic, &handle_map);

    let config: Config = match serde_wasm_bindgen::from_value(config) {
        Ok(config) => config,
        Err(err) => {
            error(&format!("{}", err));
            return String::from("Error");
        }
    };
    debug(&format!("config: {:?}", config));
    let config: CodespanConfig = config.into();

    let writer = BufferWriter::stderr(ColorChoice::AlwaysAnsi);
    let mut buffer = writer.buffer();

    term::emit(&mut buffer, &config, &file_db, &diagnostic).unwrap();

    let result = String::from_utf8_lossy(buffer.as_slice());

    result.to_string()
}
