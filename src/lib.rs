use codespan_reporting::term;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use codespan_reporting::diagnostic::{Diagnostic as CodespanDiagnostic, Label as CodespanLabel};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{BufferWriter, ColorChoice};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DisplayStyle {
    #[serde(rename = "rich")]
    Rich,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "short")]
    Short,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "displayStyle")]
    pub display_style: Option<DisplayStyle>,
    #[serde(rename = "tabWidth")]
    pub tab_width: Option<usize>,
    #[serde(rename = "startContextLines")]
    pub start_context_lines: Option<usize>,
    #[serde(rename = "endContextLines")]
    pub end_context_lines: Option<usize>,
}

impl From<Config> for codespan_reporting::term::Config {
    fn from(config: Config) -> Self {
        let mut term_config = codespan_reporting::term::Config::default();
        if let Some(display_style) = config.display_style {
            term_config.display_style = match display_style {
                DisplayStyle::Rich => codespan_reporting::term::DisplayStyle::Rich,
                DisplayStyle::Medium => codespan_reporting::term::DisplayStyle::Medium,
                DisplayStyle::Short => codespan_reporting::term::DisplayStyle::Short,
            };
        }
        if let Some(tab_width) = config.tab_width {
            term_config.tab_width = tab_width;
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
pub enum Severity {
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
pub enum LabelStyle {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub style: LabelStyle,
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "rangeStart")]
    pub range_start: usize,
    #[serde(rename = "rangeEnd")]
    pub range_end: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    #[serde(default = "Vec::new")]
    pub labels: Vec<Label>,
    #[serde(default = "Vec::new")]
    pub notes: Vec<String>,
}

impl From<Diagnostic> for CodespanDiagnostic<usize> {
    fn from(diagnostic: Diagnostic) -> Self {
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
        codespan_diagnostic = codespan_diagnostic.with_labels(
            diagnostic
                .labels
                .into_iter()
                .map(|label| {
                    let mut codespan_label = match label.style {
                        LabelStyle::Primary => CodespanLabel::primary(
                            // label.file_id,
                            0,
                            label.range_start..label.range_end,
                        ),
                        LabelStyle::Secondary => CodespanLabel::secondary(
                            // label.file_id,
                            0,
                            label.range_start..label.range_end,
                        ),
                    };
                    codespan_label = codespan_label.with_message(label.message);
                    codespan_label
                })
                .collect(),
        );
        codespan_diagnostic = codespan_diagnostic.with_notes(diagnostic.notes);
        codespan_diagnostic
    }
}

#[wasm_bindgen]
pub fn emit(code: &str, diagnostic: JsValue, config: JsValue) -> String {
    // `files::SimpleFile` and `files::SimpleFiles` help you get up and running with
    // `codespan-reporting` quickly! More complicated use cases can be supported
    // by creating custom implementations of the `files::Files` trait.

    let mut files = SimpleFiles::new();

    let file_id = files.add(
        "FizzBuzz.fun",
        unindent::unindent(
            r#"
            module FizzBuzz where

            fizz₁ : Nat → String
            fizz₁ num = case (mod num 5) (mod num 3) of
                0 0 => "FizzBuzz"
                0 _ => "Fizz"
                _ 0 => "Buzz"
                _ _ => num

            fizz₂ : Nat → String
            fizz₂ num =
                case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num
        "#,
        ),
    );

    // We normally recommend creating a custom diagnostic data type for your
    // application, and then converting that to `codespan-reporting`'s diagnostic
    // type, but for the sake of this example we construct it directly.

    let diagnostic: Diagnostic = match serde_wasm_bindgen::from_value(diagnostic) {
        Ok(diagnostic) => diagnostic,
        Err(err) => {
            log(&format!("Error: {}", err));
            return String::from("Error");
        }
    };
    log(&format!("diagnostic: {:?}", diagnostic));
    let diagnostic: CodespanDiagnostic<usize> = diagnostic.into();

    let config: Config = match serde_wasm_bindgen::from_value(config) {
        Ok(config) => config,
        Err(err) => {
            log(&format!("Error: {}", err));
            return String::from("Error");
        }
    };
    log(&format!("config: {:?}", config));

    let writer = BufferWriter::stderr(ColorChoice::AlwaysAnsi);
    let mut buffer = writer.buffer();

    term::emit(&mut buffer, &config.into(), &files, &diagnostic).unwrap();

    let result = String::from_utf8_lossy(buffer.as_slice());

    result.to_string()
}
