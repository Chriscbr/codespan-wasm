/**
 * The display style to use when rendering diagnostics.
 */
export type DisplayStyle = "rich" | "medium" | "short";

/**
 * Configures how a diagnostic is rendered.
 */
export interface Config {
  /**
   * The display style to use when rendering diagnostics. Defaults to "rich".
   */
  displayStyle?: DisplayStyle;
  /**
   * Column width of tabs. Defaults to: 4.
   */
  tabWidth?: number;
  /**
   * The minimum number of lines to be shown after the line on which a multiline
   * label begins. Defaults to: 3.
   */
  startContextLines?: number;
  /**
   * The minimum number of lines to be shown before the line on which a
   * multiline Label ends. Defaults to: 1.
   */
  endContextLines?: number;
}

/**
 * A severity level for diagnostic messages.
 */
export type Severity = "bug" | "error" | "warning" | "note" | "help";

/**
 * The style of the label.
 * 
 * Primary labels are used to describe the primary cause of a diagnostic.
 * Secondary labels are used to provide additional context for a diagnostic.
 */
export type LabelStyle = "primary" | "secondary";

/**
 * A label describing an underlined region of code associated with a diagnostic.
 */
export interface Label {
  /**
   * The style of the label.
   */
  style: LabelStyle;
  /**
   * The file that we are labelling.
   */
  fileId: string; // TODO?
  /**
   * The start of the range in bytes we are going to include in the final
   * snippet.
   */
  rangeStart: number;
  /**
   * The end of the range in bytes we are going to include in the final snippet.
   */
  rangeEnd: number;
  /**
   * An optional message to provide some additional information for the
   * underlined code. These should not include line breaks.
   */
  message: string;
}

/**
 * Represents a diagnostic message that can provide information like errors and
 * warnings to the user.
 *
 * The position of a Diagnostic is considered to be the position of the Label
 * that has the earliest starting position and has the highest style which
 * appears in all the labels of the diagnostic.
 */
export interface Diagnostic {
  /**
   * The overall severity of the diagnostic.
   */
  severity: Severity;
  /**
   * An optional code that identifies this diagnostic.
   */
  code?: string;
  /**
   * The main message associated with this diagnostic.
   *
   * These should not include line breaks, and in order support the ‘short’
   * diagnostic display mod, the message should be specific enough to make sense
   * on its own, without additional context provided by labels and notes.
   */
  message: string;
  /**
   * Source labels that describe the cause of the diagnostic. The order of the
   * labels inside the vector does not have any meaning. The labels are always
   * arranged in the order they appear in the source code.
   */
  labels?: Label[];
  /**
   * Notes that are associated with the primary cause of the diagnostic. These
   * can include line breaks for improved formatting.
   */
  notes?: string[];
}

/**
 * A file referenced by a diagnostic.
 */
export interface File {
  /**
   * The name of the file. This is the same as the fileId used to identify the
   * file in the diagnostic.
   */
  name: string;
  /**
   * The source code of the file.
   */
  source: string;
}

const codespan_bindings = import("../pkg");

export async function emit(
  files: File[],
  diagnostic: Diagnostic,
  config: Config,
): Promise<string> {
  return codespan_bindings.then((mod) => {
    return mod.emit(files, diagnostic, config);
  });
}
