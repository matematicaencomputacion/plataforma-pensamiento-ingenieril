import { component$ } from "@builder.io/qwik";

export type ResourceLink = {
  id: string;
  label: string;
  href: string;
  external?: boolean;
};

/** Botonera de recursos del curso (4 slots; el 4.º a la derecha = tutorial GitHub). */
export const COURSE_RESOURCE_LINKS: ResourceLink[] = [
  {
    id: "course-video",
    label: "Curso Python",
    href: "https://www.youtube.com/watch?v=Kp4Mvapo5kc",
    external: true,
  },
  {
    id: "github-repo",
    label: "GitHub Hello-Python",
    href: "https://github.com/mouredev/Hello-Python",
    external: true,
  },
  {
    id: "python-docs",
    label: "Docs Python",
    href: "https://docs.python.org/es/3/tutorial/",
    external: true,
  },
  {
    id: "github-tutorial",
    label: "Tutorial GitHub",
    href: "https://www.youtube.com/watch?v=3GymExBkKjE",
    external: true,
  },
];

export const ResourceLinks = component$(() => {
  return (
    <nav class="resource-links" aria-label="Recursos del curso">
      <p class="resource-links__label">Recursos</p>
      <ul class="resource-links__list">
        {COURSE_RESOURCE_LINKS.map((link) => (
          <li key={link.id} class="resource-links__item">
            <a
              class={`resource-links__btn${
                link.id === "github-tutorial"
                  ? " resource-links__btn--accent"
                  : ""
              }`}
              href={link.href}
              target="_blank"
              rel="noopener noreferrer"
              data-resource-id={link.id}
            >
              {link.label}
            </a>
          </li>
        ))}
      </ul>
    </nav>
  );
});
