export type DisplayStyle = "rich" | "medium" | "short";

export interface Config {
  displayStyle?: DisplayStyle;
  tabWidth?: number;
  startContextLines?: number;
  endContextLines?: number;
}

export async function emit(
  code: string,
  config: Config,
): Promise<string> {
  return import("../pkg").then((mod) => {
    return mod.emit(code, config);
  });
}

emit("let x = 1 + 1;", {
  displayStyle: "rich",
}).then((result) => {
  console.log(result);
}).catch((err) => {
  console.error(err);
});