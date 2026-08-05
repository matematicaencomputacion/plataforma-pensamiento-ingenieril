/** Normalización de URLs de YouTube hacia el formato embed + IFrame API. */

const YOUTUBE_ID_RE = /^[\w-]{11}$/;

export type YouTubeEmbedOptions = {
  autoplay?: boolean;
  enableJsApi?: boolean;
  origin?: string;
  rel?: boolean;
  playsInline?: boolean;
  modestBranding?: boolean;
};

/**
 * Extrae el VIDEO_ID de watch, youtu.be, embed, shorts o un ID crudo.
 */
export function extractYouTubeVideoId(input: string): string | null {
  const raw = input.trim();
  if (!raw) {
    return null;
  }

  if (YOUTUBE_ID_RE.test(raw)) {
    return raw;
  }

  try {
    const url = new URL(raw);
    const host = url.hostname.replace(/^www\./, "").toLowerCase();

    if (host === "youtu.be") {
      const id = url.pathname.split("/").filter(Boolean)[0] ?? "";
      return YOUTUBE_ID_RE.test(id) ? id : null;
    }

    if (host === "youtube.com" || host === "m.youtube.com" || host === "music.youtube.com") {
      const fromQuery = url.searchParams.get("v");
      if (fromQuery && YOUTUBE_ID_RE.test(fromQuery)) {
        return fromQuery;
      }

      const parts = url.pathname.split("/").filter(Boolean);
      // /embed/ID, /shorts/ID, /live/ID, /v/ID
      if (
        parts.length >= 2 &&
        ["embed", "shorts", "live", "v"].includes(parts[0]) &&
        YOUTUBE_ID_RE.test(parts[1])
      ) {
        return parts[1];
      }
    }
  } catch {
    // no es URL absoluta; intentar sacar un ID al final del string
    const maybe = raw.split(/[?&#/]/).filter(Boolean).pop() ?? "";
    if (YOUTUBE_ID_RE.test(maybe)) {
      return maybe;
    }
  }

  return null;
}

/**
 * Construye una URL de incrustación limpia para iframe / IFrame API.
 * Ejemplo: https://www.youtube.com/embed/VIDEO_ID?autoplay=0&enablejsapi=1
 */
export function toYouTubeEmbedUrl(
  input: string,
  options: YouTubeEmbedOptions = {},
): string | null {
  const videoId = extractYouTubeVideoId(input);
  if (!videoId) {
    return null;
  }

  const params = new URLSearchParams();
  params.set("autoplay", options.autoplay ? "1" : "0");
  params.set("enablejsapi", options.enableJsApi === false ? "0" : "1");
  params.set("rel", options.rel ? "1" : "0");
  params.set("playsinline", options.playsInline === false ? "0" : "1");
  params.set("modestbranding", options.modestBranding === false ? "0" : "1");
  if (options.origin) {
    params.set("origin", options.origin);
  }

  return `https://www.youtube.com/embed/${videoId}?${params.toString()}`;
}

export function isValidYouTubeResource(input: string | null | undefined): boolean {
  return Boolean(input && extractYouTubeVideoId(input));
}
