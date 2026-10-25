import { format } from "prettier";
function cell(value) {
  return String(value ?? "")
    .replaceAll("|", "\\|")
    .replaceAll("\n", " ");
}
function table(headers, rows) {
  return [
    `| ${headers.join(" | ")} |`,
    `| ${headers.map(() => "---").join(" | ")} |`,
    ...rows.map((row) => `| ${row.map(cell).join(" | ")} |`),
  ].join("\n");
}
export async function commandReference(discovery) {
  const sections = [
    "# CanvaToPSD CLI command reference",
    `Generated from Rust CLI ${discovery.version} with \`canvatopsd discover\`. Run \`npm run doc:commands\` to update this file. Do not edit command tables by hand.`,
    "Use this page to inspect every command without installing or executing the CLI. For capabilities, installation availability and pricing boundaries, read [product facts](product-facts.md). For task recovery and credential handling, read [Agent guidance](agent-discovery.md).",
    "Business commands return JSON on stdout; help and version are text. Diagnostics use stderr. Success exits 0; failure exits 1. No `--json` option is needed. Use `--help` on a command for terminal help; generated help aliases are omitted below.",
    "## Global options",
    table(
      ["Option", "Description", "Default"],
      discovery.command.options.map((option) => [
        `\`${option.flags}\``,
        option.description,
        option.default ?? "—",
      ]),
    ),
  ];
  function visit(command, parents) {
    const path = [...parents, command.name];
    sections.push(`## canvatopsd ${path.join(" ")}`, command.description || "");
    const usage = [
      "canvatopsd",
      ...path,
      ...command.arguments.map((argument) =>
        argument.required ? `<${argument.name}>` : `[${argument.name}]`,
      ),
    ];
    if (command.commands.length) usage.push("<COMMAND>");
    for (const option of command.options.filter((option) => option.required))
      usage.push(option.flags);
    if (
      command.options.some(
        (option) =>
          !option.required && !["--help", "--version"].includes(option.flags),
      )
    )
      usage.push("[OPTIONS]");
    sections.push("```text\n" + usage.join(" ") + "\n```");
    if (command.arguments.length)
      sections.push(
        table(
          ["Argument", "Required", "Description"],
          command.arguments.map((argument) => [
            `\`${argument.name}\``,
            argument.required ? "yes" : "no",
            argument.description || "See command description.",
          ]),
        ),
      );
    const ownOptions = command.options.filter(
      (option) =>
        !discovery.command.options.some(
          (global) => global.flags === option.flags,
        ),
    );
    if (ownOptions.length)
      sections.push(
        table(
          ["Option", "Required", "Default", "Description / allowed values"],
          ownOptions.map((option) => [
            `\`${option.flags}\``,
            option.required ? "yes" : "no",
            option.default ?? "—",
            [
              option.description,
              option.choices?.length
                ? `Allowed: ${option.choices.join(", ")}`
                : "",
            ]
              .filter(Boolean)
              .join(" "),
          ]),
        ),
      );
    for (const child of command.commands.filter(
      (child) => child.name !== "help",
    ))
      visit(child, path);
  }
  for (const command of discovery.command.commands.filter(
    (command) => command.name !== "help",
  ))
    visit(command, []);
  return format(sections.join("\n\n") + "\n", { parser: "markdown" });
}
