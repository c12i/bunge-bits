export const highlightText = (text: string, term: string) => {
  if (!term || typeof term !== "string") return text;

  const pattern = new RegExp(`\\b(${escapeRegExp(term)})\\b`, "gi");

  return text
    .split(pattern)
    .map((part, i) =>
      part.toLowerCase() === term.toLowerCase() ? <mark key={i}>{part}</mark> : part
    );
};

const escapeRegExp = (str: string = "") => {
  return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
};
