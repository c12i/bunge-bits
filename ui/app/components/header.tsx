import { Link } from "@remix-run/react";

const Header = () => {
  return (
    <header className="bg-white/80 backdrop-blur-sm border-b border-gray-200 sticky top-0 z-50">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <Link to="/" className="flex items-center space-x-3">
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

          <nav className="flex items-center space-x-6">
            <Link
              to="/about"
              className="text-gray-600 hover:text-red-800 transition-colors font-medium"
            >
              About
            </Link>
          </nav>
        </div>
      </div>
    </header>
  );
};

export default Header;
