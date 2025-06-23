import React from "react";

export const highlightText = (text: string, terms: string[]) => {
  if (!terms || !terms.length) return text;

  const pattern = new RegExp(`(${terms.map((t) => escapeRegExp(t)).join("|")})`, "gi");
  return text.split(pattern).map((part, i) =>
    terms.some((term) => part.toLowerCase() === term.toLowerCase()) ? (
      <mark key={i} className="bg-yellow-200 px-1">
        {part}
      </mark>
    ) : (
      part
    )
  );
};

export const highlightChildren = (children: React.ReactNode, terms: string[]) => {
  return React.Children.map(children, (child) => {
    if (typeof child === "string") {
      return highlightText(child, terms);
    }
    return child;
  });
};

const escapeRegExp = (str: string) => {
  return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
};
