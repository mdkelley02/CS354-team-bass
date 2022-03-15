document.querySelectorAll("pre").forEach((element) => {
  hljs.highlightElement(element);
  element.classList.add("code-snippet");
});
