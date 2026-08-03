import { component$ } from "@builder.io/qwik";
import { trackLabel, type Level } from "../../lib/api";

type LevelDescriptionProps = {
  level: Level;
};

export const LevelDescription = component$<LevelDescriptionProps>((props) => {
  const isMicroPaso = props.level.track_type === "micro_paso";

  return (
    <article
      class={`level-card${isMicroPaso ? " level-card--micro" : " level-card--reto"}`}
      aria-labelledby="level-title"
    >
      <div class="level-card__meta">
        <span
          class={`level-card__badge${
            isMicroPaso
              ? " level-card__badge--micro"
              : " level-card__badge--reto"
          }`}
        >
          {trackLabel(props.level.track_type)}
        </span>
        <span class="level-card__id">Nivel {props.level.id}</span>
      </div>

      <h2 id="level-title" class="level-card__title">
        {props.level.title}
      </h2>

      <p class="level-card__statement">{props.level.statement}</p>

      <p class="level-card__hint">
        {isMicroPaso
          ? "Sigue las instrucciones paso a paso y enfócate en la sintaxis."
          : "Plantea una solución de ingeniería: abstrae, diseña y valida con un print()."}
      </p>
    </article>
  );
});
