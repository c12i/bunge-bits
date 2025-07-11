import { RemixI18Next } from "remix-i18next/server";

import i18n from "./i18n/i18n";

export default new RemixI18Next({
  detection: {
    supportedLanguages: i18n.supportedLngs,
    fallbackLanguage: i18n.fallbackLng,
    order: ["searchParams", "header"],
    searchParamKey: "lng",
  },
  i18next: {
    ...i18n,
  },
});
