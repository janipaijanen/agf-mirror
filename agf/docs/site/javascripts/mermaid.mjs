import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11.16.0/dist/mermaid.esm.min.mjs";

const initialize = mermaid.initialize.bind(mermaid);
const render = mermaid.render.bind(mermaid);

// Material initializes Mermaid again for its palette-aware theme CSS. Preserve
// the responsive configuration across every such initialization instead of
// relying on the first call's values to survive.
mermaid.initialize = (configuration = {}) =>
  initialize({
    ...configuration,
    startOnLoad: false,
    securityLevel: "loose",
    flowchart: {
      ...configuration.flowchart,
      useMaxWidth: false,
    },
    sequence: {
      ...configuration.sequence,
      useMaxWidth: false,
    },
  });

mermaid.initialize();

// Material mounts Mermaid's SVG in a closed shadow root. Chromium's internal
// tree resolves Mermaid's title/description ID references there, but its
// AT-SPI bridge does not. Copy the same text to direct ARIA attributes so the
// accessible name and description survive that boundary for screen readers.
mermaid.render = async (...arguments_) => {
  const result = await render(...arguments_);
  const document_ = new DOMParser().parseFromString(
    result.svg,
    "image/svg+xml",
  );
  const svg = document_.documentElement;
  const title = svg.querySelector("title")?.textContent?.trim();
  const description = svg.querySelector("desc")?.textContent?.trim();

  svg.setAttribute("role", "img");
  if (title) svg.setAttribute("aria-label", title);
  if (description) svg.setAttribute("aria-description", description);
  svg.removeAttribute("aria-labelledby");
  svg.removeAttribute("aria-describedby");

  return {
    ...result,
    svg: new XMLSerializer().serializeToString(svg),
  };
};

// Material for MkDocs discovers this object and handles full-page and instant
// navigation rendering, including its palette-aware Mermaid configuration.
window.mermaid = mermaid;
