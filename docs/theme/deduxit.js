// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

// The tab's icon is the logo's square form, which deduxit's web page serves
// in the scheme of the system's.
(() => {
  const icon = document.querySelector('link[rel="icon"]');
  const dark = window.matchMedia("(prefers-color-scheme: dark)");
  const set = () => {
    icon.href = `https://deduxit.github.io/logo/square-${dark.matches ? "dark" : "light"}.svg`;
  };
  set();
  dark.addEventListener("change", set);
})();
