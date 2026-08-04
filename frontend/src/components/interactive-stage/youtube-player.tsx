import {
  component$,
  noSerialize,
  useSignal,
  useVisibleTask$,
  type NoSerialize,
  type QRL,
} from "@builder.io/qwik";

type YTPlayer = {
  seekTo: (seconds: number, allowSeekAhead: boolean) => void;
  getCurrentTime: () => number;
  destroy: () => void;
};

type YTNamespace = {
  Player: new (
    elementId: string,
    config: {
      videoId: string;
      playerVars?: Record<string, string | number>;
      events?: {
        onReady?: () => void;
        onStateChange?: (event: { data: number }) => void;
      };
    },
  ) => YTPlayer;
  PlayerState: { PLAYING: number };
};

declare global {
  interface Window {
    YT?: YTNamespace;
    onYouTubeIframeAPIReady?: () => void;
  }
}

export type YouTubePlayerProps = {
  videoId: string;
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
    if (!document.querySelector('script[src="https://www.youtube.com/iframe_api"]')) {
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

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async ({ track, cleanup }) => {
    const videoId = track(() => props.videoId);
    const domId = track(() => props.playerDomId);

    await loadYouTubeAPI();
    if (!window.YT?.Player) {
      return;
    }

    const existing = playerRef.value;
    if (existing) {
      existing.destroy();
      playerRef.value = null;
    }

    const player = new window.YT.Player(domId, {
      videoId,
      playerVars: {
        enablejsapi: 1,
        rel: 0,
        modestbranding: 1,
        playsinline: 1,
      },
      events: {
        onReady: () => {
          ready.value = true;
        },
      },
    });
    playerRef.value = noSerialize(player);

    const timer = window.setInterval(() => {
      const current = playerRef.value;
      if (!current || !props.onTimeUpdate$) {
        return;
      }
      try {
        const t = current.getCurrentTime();
        void props.onTimeUpdate$(t);
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

  return (
    <div class="yt-player">
      <div id={props.playerDomId} class="yt-player__frame" />
    </div>
  );
});

export function seekYouTubePlayer(
  player: YTPlayer | null | undefined,
  seconds: number,
) {
  player?.seekTo(seconds, true);
}
