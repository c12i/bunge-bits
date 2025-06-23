import { Link } from "@remix-run/react";

export default function NotFoundPage() {
  return (
    <div className="bg-gradient-to-br from-red-50 to-orange-50">
      <main className="container mx-auto px-4 py-8 max-w-6xl">
        <div className="text-center">
          <h1 className="text-xl font-bold text-red-700">404</h1>
          <p className="text-xl mt-4">Page not found</p>
          <p className="mt-2 text-gray-600">
            Sorry, we couldn't find what you were looking for.
          </p>
          <Link to="/" className="mt-6 inline-block text-red-800 hover:underline">
            Go home
          </Link>
        </div>
      </main>
    </div>
  );
}
