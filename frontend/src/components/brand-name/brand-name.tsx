import { component$ } from "@builder.io/qwik";

/** Marca tipográfica: IngenierIA (últimas dos letras IA en acento y mayúsculas). */
export const BrandName = component$(() => {
  return (
    <span class="brand-name">
      Ingenier
      <span class="brand-name__mark">IA</span>
    </span>
  );
});

/** Texto plano para <title>, aria y metadatos. */
export const BRAND_NAME_PLAIN = "IngenierIA";
