import { component$, type QRL } from "@builder.io/qwik";
import type { AdaptiveMcq } from "../../lib/microsteps";
import { AdaptiveMcqPanel } from "./adaptive-mcq";
import { parsePromptWithInlineMcq, splitInlineMarkdown } from "./prompt-parse";

const InlineRichText = component$((props: { text: string }) => {
  const parts = splitInlineMarkdown(props.text);
  return (
    <>
      {parts.map((part, i) => {
        if (part.kind === "bold") {
          return (
            <strong key={`b-${i}`} class="prompt-md__bold">
              {part.value}
            </strong>
          );
        }
        if (part.kind === "code") {
          return (
            <code key={`c-${i}`} class="prompt-md__code">
              {part.value}
            </code>
          );
        }
        return <span key={`t-${i}`}>{part.value}</span>;
      })}
    </>
  );
});

export type PromptMarkdownProps = {
  markdown: string;
  adaptiveMcq?: AdaptiveMcq;
  selectedMcqKey?: string | null;
  onMcqSelect$?: QRL<(key: string) => void>;
};

/**
 * Enunciado: markdown ligero + Check rápido adaptativo (si hay semilla).
 */
export const PromptMarkdown = component$((props: PromptMarkdownProps) => {
  const parsed = parsePromptWithInlineMcq(props.markdown);
  // Preferir opciones tipadas de la semilla; fallback a líneas - a) del markdown.
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
                    <InlineRichText text={ln.replace(/^\s*[-*]\s+/, "")} />
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
                <InlineRichText text={ln} />
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
