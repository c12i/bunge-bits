import { Link, useLocation } from "@remix-run/react";
import { Languages } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "./ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";

export default function LanguageSwitcher() {
  const { t, i18n } = useTranslation();
  const location = useLocation();
  const currentLanguage = i18n.language;

  const createLanguageUrl = (lang: string) => {
    const searchParams = new URLSearchParams(location.search);
    searchParams.set("lng", lang);
    return `${location.pathname}?${searchParams.toString()}`;
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="sm" className="bg-white text-gray-700">
          <Languages className="h-4 w-4 mr-2" />
          {currentLanguage === "sw" ? t("language.swahili") : t("language.english")}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem asChild>
          <Link
            to={createLanguageUrl("en")}
            className={`w-full text-left ${currentLanguage === "en" ? "bg-gray-100" : ""}`}
          >
            {t("language.english")}
          </Link>
        </DropdownMenuItem>
        <DropdownMenuItem asChild>
          <Link
            to={createLanguageUrl("sw")}
            className={`w-full text-left ${currentLanguage === "sw" ? "bg-gray-100" : ""}`}
          >
            {t("language.swahili")}
          </Link>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
