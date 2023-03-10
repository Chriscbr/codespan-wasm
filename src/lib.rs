use codespan_reporting::term;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{BufferWriter, ColorChoice};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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

#[derive(Serialize, Deserialize, Debug)]
pub enum DisplayStyle {
    #[serde(rename = "rich")]
    Rich,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "short")]
    Short,
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

#[wasm_bindgen]
pub fn emit(code: &str, config: JsValue) -> String {
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

    let diagnostic = Diagnostic::error()
        .with_message("`case` clauses have incompatible types")
        .with_code("E0308")
        .with_labels(vec![
            Label::primary(file_id, 328..331).with_message("expected `String`, found `Nat`"),
            Label::secondary(file_id, 211..331)
                .with_message("`case` clauses have incompatible types"),
            Label::secondary(file_id, 258..268)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 284..290)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 306..312)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 186..192).with_message("expected type `String` found here"),
        ])
        .with_notes(vec![unindent::unindent(
            "
            expected type `String`
                found type `Nat`
        ",
        )]);

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
    let config = config.into();

    term::emit(&mut buffer, &config, &files, &diagnostic).unwrap();

    let result = String::from_utf8_lossy(buffer.as_slice());

    result.to_string()
}
