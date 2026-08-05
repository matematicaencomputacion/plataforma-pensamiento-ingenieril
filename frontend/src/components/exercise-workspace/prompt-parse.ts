/**
 * Utilidades de parseo para el panel Enunciado (markdown ligero + opciones a/b/c).
 * Consumido por `prompt-markdown.tsx`.
 */

export type InlineMcqOption = {
  key: string;
  label: string;
};

export type ParsedPrompt = {
  body: string;
  options: InlineMcqOption[];
};

const OPTION_LINE_RE = /^[-*]\s*([a-cA-C])\)\s+(.+)$/;

/** Separa cuerpo del prompt e opciones tipo `- a) …`. */
export function parsePromptWithInlineMcq(markdown: string): ParsedPrompt {
  const lines = markdown.split("\n");
  const bodyLines: string[] = [];
  const options: InlineMcqOption[] = [];

  for (const line of lines) {
    const match = OPTION_LINE_RE.exec(line.trim());
    if (match) {
      options.push({
        key: match[1].toLowerCase(),
        label: match[2].trim(),
      });
      continue;
    }
    bodyLines.push(line);
  }

  return {
    body: bodyLines.join("\n").replace(/\n{3,}/g, "\n\n").trimEnd(),
    options,
  };
}

export type TextPart =
  | { kind: "text"; value: string }
  | { kind: "bold"; value: string }
  | { kind: "code"; value: string };

/** Partes inline: `code` y **negrita**. */
export function splitInlineMarkdown(text: string): TextPart[] {
  const parts: TextPart[] = [];
  const re = /(\*\*[^*]+\*\*|`[^`]+`)/g;
  let last = 0;
  let match: RegExpExecArray | null;
  while ((match = re.exec(text)) !== null) {
    if (match.index > last) {
      parts.push({ kind: "text", value: text.slice(last, match.index) });
    }
    const token = match[0];
    if (token.startsWith("**") && token.endsWith("**")) {
      parts.push({ kind: "bold", value: token.slice(2, -2) });
    } else if (token.startsWith("`") && token.endsWith("`")) {
      parts.push({ kind: "code", value: token.slice(1, -1) });
    } else {
      parts.push({ kind: "text", value: token });
    }
    last = match.index + token.length;
  }
  if (last < text.length) {
    parts.push({ kind: "text", value: text.slice(last) });
  }
  if (parts.length === 0) {
    parts.push({ kind: "text", value: text });
  }
  return parts;
}
