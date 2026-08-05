import {
  component$,
  noSerialize,
  useSignal,
  useVisibleTask$,
  type NoSerialize,
  type QRL,
} from "@builder.io/qwik";
import {
  extractYouTubeVideoId,
  toYouTubeEmbedUrl,
} from "../../lib/youtube-utils";

type YTPlayer = {
  seekTo: (seconds: number, allowSeekAhead: boolean) => void;
  getCurrentTime: () => number;
  destroy: () => void;
};

type YTNamespace = {
  Player: new (
    elementId: string | HTMLElement,
    config?: {
      events?: {
        onReady?: () => void;
        onError?: (event: { data: number }) => void;
      };
    },
  ) => YTPlayer;
};

declare global {
  interface Window {
    YT?: YTNamespace;
    onYouTubeIframeAPIReady?: () => void;
  }
}

export type YouTubePlayerProps = {
  /** URL watch / youtu.be / embed / shorts o VIDEO_ID crudo del curriculum. */
  resourceUrl: string;
  playerDomId: string;
  onTimeUpdate$?: QRL<(seconds: number) => void>;
  seekRequest?: number | null;
};

let apiLoader: Promise<void> | null = null;

function loadYouTubeAPI(): Promise<void> {
  if (typeof window === "undefined") {
    return Promise.resolve();
  }
  if (window.YT?.Player) {
    return Promise.resolve();
  }
  if (apiLoader) {
    return apiLoader;
  }
  apiLoader = new Promise((resolve) => {
    const previous = window.onYouTubeIframeAPIReady;
    window.onYouTubeIframeAPIReady = () => {
      previous?.();
      resolve();
    };
    if (
      !document.querySelector('script[src="https://www.youtube.com/iframe_api"]')
    ) {
      const script = document.createElement("script");
      script.src = "https://www.youtube.com/iframe_api";
      script.async = true;
      document.head.appendChild(script);
    }
  });
  return apiLoader;
}

export const YouTubePlayer = component$<YouTubePlayerProps>((props) => {
  const playerRef = useSignal<NoSerialize<YTPlayer> | null>(null);
  const ready = useSignal(false);
  const apiError = useSignal("");

  const videoId = extractYouTubeVideoId(props.resourceUrl);
  const embedSrc = videoId
    ? toYouTubeEmbedUrl(props.resourceUrl, {
        autoplay: false,
        enableJsApi: true,
      })
    : null;

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async ({ track, cleanup }) => {
    const resourceUrl = track(() => props.resourceUrl);
    const domId = track(() => props.playerDomId);

    ready.value = false;
    apiError.value = "";

    const id = extractYouTubeVideoId(resourceUrl);
    const withOrigin = toYouTubeEmbedUrl(resourceUrl, {
      autoplay: false,
      enableJsApi: true,
      origin: window.location.origin,
    });

    if (!id || !withOrigin) {
      apiError.value =
        "No se pudo interpretar la URL del video. Revisá el recurso del curriculum.";
      return;
    }

    const iframe = document.getElementById(domId) as HTMLIFrameElement | null;
    if (!iframe) {
      apiError.value = "No se encontró el contenedor del reproductor.";
      return;
    }

    // Garantiza src embed (nunca watch?v=) e incluye origin para la IFrame API.
    if (iframe.src !== withOrigin) {
      iframe.src = withOrigin;
    }

    await loadYouTubeAPI();
    if (!window.YT?.Player) {
      apiError.value = "No se pudo cargar la API de YouTube.";
      return;
    }

    try {
      playerRef.value?.destroy();
    } catch {
      // ignore
    }
    playerRef.value = null;

    const player = new window.YT.Player(iframe, {
      events: {
        onReady: () => {
          ready.value = true;
        },
        onError: () => {
          apiError.value =
            "YouTube no pudo reproducir este video (ID inválido o sin permiso de incrustación).";
        },
      },
    });
    playerRef.value = noSerialize(player);

    const timer = window.setInterval(() => {
      const current = playerRef.value;
      if (!current || !props.onTimeUpdate$ || !ready.value) {
        return;
      }
      try {
        void props.onTimeUpdate$(current.getCurrentTime());
      } catch {
        // player aún no listo
      }
    }, 250);

    cleanup(() => {
      window.clearInterval(timer);
      try {
        playerRef.value?.destroy();
      } catch {
        // ignore
      }
      playerRef.value = null;
      ready.value = false;
    });
  });

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(({ track }) => {
    const seekTo = track(() => props.seekRequest);
    const player = playerRef.value;
    if (seekTo == null || !player || !ready.value) {
      return;
    }
    player.seekTo(seekTo, true);
  });

  if (!embedSrc || !videoId) {
    return (
      <div class="yt-player yt-player--error" role="alert">
        <p class="yt-player__error">
          URL de video inválida o no compatible con incrustación. Se espera un
          enlace de YouTube (`watch`, `youtu.be`, `embed`) o un VIDEO_ID.
        </p>
      </div>
    );
  }

  return (
    <div class="yt-player">
      <iframe
        key={videoId}
        id={props.playerDomId}
        class="yt-player__frame"
        src={embedSrc}
        title="Reproductor de lección YouTube"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        allowFullscreen
        referrerPolicy="strict-origin-when-cross-origin"
      />
      {apiError.value && (
        <p class="yt-player__error yt-player__error--overlay" role="status">
          {apiError.value}
        </p>
      )}
    </div>
  );
});
