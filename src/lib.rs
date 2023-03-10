use codespan_reporting::term;
use wasm_bindgen::prelude::*;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{BufferWriter, ColorChoice};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
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

    // We now set up the writer and configuration, and then finally render the
    // diagnostic to standard error.

    let writer = BufferWriter::stderr(ColorChoice::AlwaysAnsi);
    let mut buffer = writer.buffer();
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut buffer, &config, &files, &diagnostic).unwrap();

    let stuff = String::from_utf8_lossy(buffer.as_slice());

    stuff.to_string()
}
