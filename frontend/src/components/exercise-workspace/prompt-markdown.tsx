import { component$, type QRL } from "@builder.io/qwik";
import type { AdaptiveMcq } from "../../lib/microsteps";
import { AdaptiveMcqPanel } from "./adaptive-mcq";
import { parsePromptWithInlineMcq, splitInlineMarkdown } from "./prompt-parse";
import type { VariableHoverHint } from "./python-type-catalog";

const HintedText = component$((props: {
  text: string;
  hints?: Record<string, VariableHoverHint>;
  /** Si true, no envuelve el token en <code> (ya está dentro de uno). */
  nested?: boolean;
}) => {
  const hints = props.hints;
  if (!hints || !/\b(nombre|edad)\b/.test(props.text)) {
    return <>{props.text}</>;
  }

  const pieces: Array<{ key: string; kind: "text" | "hint"; value: string }> =
    [];
  const re = /\b(nombre|edad)\b/g;
  let last = 0;
  let match: RegExpExecArray | null;
  let i = 0;
  while ((match = re.exec(props.text)) !== null) {
    if (match.index > last) {
      pieces.push({
        key: `t-${i++}`,
        kind: "text",
        value: props.text.slice(last, match.index),
      });
    }
    pieces.push({ key: `h-${i++}`, kind: "hint", value: match[1] });
    last = match.index + match[1].length;
  }
  if (last < props.text.length) {
    pieces.push({
      key: `t-${i++}`,
      kind: "text",
      value: props.text.slice(last),
    });
  }

  return (
    <>
      {pieces.map((piece) => {
        if (piece.kind === "text") {
          return <span key={piece.key}>{piece.value}</span>;
        }
        const hint = hints[piece.value];
        if (!hint) {
          return <span key={piece.key}>{piece.value}</span>;
        }
        return (
          <span key={piece.key} class="prompt-md__var-tip" tabIndex={0}>
            {props.nested ? (
              <span class="prompt-md__var-token">{piece.value}</span>
            ) : (
              <code class="prompt-md__code prompt-md__code--hinted">
                {piece.value}
              </code>
            )}
            <span class="prompt-md__var-pop" role="tooltip">
              <strong class="prompt-md__var-pop-type">{hint.typeLabel}</strong>
              <span class="prompt-md__var-pop-blurb">{hint.blurb}</span>
            </span>
          </span>
        );
      })}
    </>
  );
});

const InlineRichText = component$((props: {
  text: string;
  variableHints?: Record<string, VariableHoverHint>;
}) => {
  const parts = splitInlineMarkdown(props.text);
  return (
    <>
      {parts.map((part, i) => {
        if (part.kind === "bold") {
          return (
            <strong key={`b-${i}`} class="prompt-md__bold">
              <HintedText text={part.value} hints={props.variableHints} />
            </strong>
          );
        }
        if (part.kind === "code") {
          return (
            <code key={`c-${i}`} class="prompt-md__code">
              <HintedText
                text={part.value}
                hints={props.variableHints}
                nested
              />
            </code>
          );
        }
        return (
          <span key={`t-${i}`}>
            <HintedText text={part.value} hints={props.variableHints} />
          </span>
        );
      })}
    </>
  );
});

export type PromptMarkdownProps = {
  markdown: string;
  adaptiveMcq?: AdaptiveMcq;
  selectedMcqKey?: string | null;
  onMcqSelect$?: QRL<(key: string) => void>;
  variableHints?: Record<string, VariableHoverHint>;
};

/**
 * Enunciado: markdown ligero + hints hover de variables (opcional).
 */
export const PromptMarkdown = component$((props: PromptMarkdownProps) => {
  const parsed = parsePromptWithInlineMcq(props.markdown);
  const adaptiveFromLines: AdaptiveMcq | undefined =
    !props.adaptiveMcq && parsed.options.length > 0
      ? {
          options: parsed.options.map((o) => ({
            key: o.key,
            text: o.label,
            feedback: "Gracias por tu respuesta. Continuá explorando el curso.",
            alignmentScore: 3,
          })),
        }
      : undefined;
  const mcq = props.adaptiveMcq ?? adaptiveFromLines;
  const paragraphs = parsed.body.split(/\n{2,}/);

  return (
    <div class="prompt-md">
      {paragraphs.map((block, bi) => {
        const lines = block.split("\n");
        const isList =
          lines.filter((ln) => ln.trim()).length > 0 &&
          lines
            .filter((ln) => ln.trim())
            .every((ln) => /^\s*[-*]\s+/.test(ln));
        if (isList) {
          return (
            <ul key={`ul-${bi}`} class="prompt-md__list">
              {lines
                .filter((ln) => ln.trim())
                .map((ln, li) => (
                  <li key={`li-${bi}-${li}`}>
                    <InlineRichText
                      text={ln.replace(/^\s*[-*]\s+/, "")}
                      variableHints={props.variableHints}
                    />
                  </li>
                ))}
            </ul>
          );
        }
        return (
          <p key={`p-${bi}`} class="prompt-md__p">
            {lines.map((ln, li) => (
              <span key={`ln-${bi}-${li}`}>
                {li > 0 && <br />}
                <InlineRichText
                  text={ln}
                  variableHints={props.variableHints}
                />
              </span>
            ))}
          </p>
        );
      })}

      {mcq && props.onMcqSelect$ && (
        <AdaptiveMcqPanel
          mcq={mcq}
          selectedKey={props.selectedMcqKey ?? null}
          onSelect$={props.onMcqSelect$}
        />
      )}
    </div>
  );
});
