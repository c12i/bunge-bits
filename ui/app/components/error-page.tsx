import { isRouteErrorResponse, Link } from "@remix-run/react";

type ErrorPageProps = {
  error: unknown;
};

export default function ErrorPage({ error }: ErrorPageProps) {
  const isRouteError = isRouteErrorResponse(error);
  return (
    <div className="bg-gradient-to-br from-red-50 to-orange-50">
      <main className="container mx-auto px-4 py-8 max-w-6xl">
        <div className="text-center">
          <h1 className="text-xl font-bold text-red-700">
            {isRouteError ? "404" : "500"}
          </h1>
          <p className="text-xl mt-4">
            {isRouteError ? "Page Not Found" : "Internal Server Error"}{" "}
          </p>
          <p className="mt-2 text-gray-600">
            {isRouteError
              ? "Sorry, we couldn't find what you were looking for"
              : "Something Went Wrong"}
          </p>
          {isRouteError && (
            <Link to="/" className="mt-6 inline-block text-red-800 hover:underline">
              Go home
            </Link>
          )}
        </div>
      </main>
    </div>
  );
}
