import { Link } from "@remix-run/react";
import { X } from "lucide-react";
import { useEffect, useState } from "react";

const DONATION_BANNER_KEY = "hideDonationBanner";

import LanguageSwitcher from "~/components/language-switcher";

const Header = () => {
  const [showBanner, setShowBanner] = useState(false);

  useEffect(() => {
    const hidden = localStorage.getItem(DONATION_BANNER_KEY);
    if (!hidden) {
      setShowBanner(true);
    }
  }, []);

  const dismissBanner = () => {
    localStorage.setItem(DONATION_BANNER_KEY, "true");
    setShowBanner(false);
  };

  return (
    <>
      {showBanner && (
        <div className="bg-muted text-foreground text-sm border-b border-border px-4 py-2">
          <div className="container mx-auto relative flex items-center justify-center">
            <span className="text-center">
              Help us keep Bunge Bits free and accessible.{" "}
              <a
                href="https://support-bungebits.c12i.xyz"
                target="_blank"
                rel="noopener noreferrer"
                className="underline hover:text-primary font-medium"
              >
                Support our work
              </a>
              .
            </span>
            <button
              onClick={dismissBanner}
              className="absolute right-4 text-sm text-muted-foreground hover:text-foreground transition"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        </div>
      )}

      <header className="bg-white/80 backdrop-blur-sm border-b border-border sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <Link to="/summaries" className="flex items-center space-x-3">
              <img
                src="/bunge-bits/logo_64x64.png"
                alt="Bunge Bits Logo"
                className="w-10 h-10 object-contain"
              />
              <div className="flex items-center space-x-2">
                <span className="text-xl font-bold text-gray-900">Bunge Bits</span>
                <span className="text-xs bg-red-500 text-white px-2 py-0.5 rounded-full font-semibold uppercase tracking-wide">
                  Beta
                </span>
              </div>
            </Link>
          </nav>
          <div className="flex items-center space-x-4">
            <LanguageSwitcher />
          </div>
        </div>
      </header>
    </>
  );
};

export default Header;
