import "./tailwind.css";

import type { LinksFunction, LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  useLoaderData,
  useRouteError,
} from "@remix-run/react";

import ErrorPage from "./components/error-page";
import Footer from "./components/footer";
import Header from "./components/header";
import { NavigationProgress } from "./components/navigation-progress";

export const links: LinksFunction = () => [
  {
    rel: "alternate",
    type: "application/rss+xml",
    title: "Bunge Bits RSS Feed",
    href: "/summaries/rss.xml",
  },
];

export async function loader({ request }: LoaderFunctionArgs) {
  const url = new URL(request.url);

  return Response.json({
    origin: url?.origin || "https://bungebits.ke",
    env: {
      PLAUSIBLE_BASE_URL: process.env.PUBLIC_PLAUSIBLE_BASE_URL,
      PLAUSIBLE_DOMAIN: process.env.PUBLIC_PLAUSIBLE_DOMAIN,
    },
  });
}

export const meta: MetaFunction<typeof loader> = ({ data }) => {
  const origin = data?.origin;

  return [
    { charset: "utf-8" },
    { name: "viewport", content: "width=device-width, initial-scale=1" },
    { title: "Bunge Bits - Legislative Summaries for the Parliament of Kenya" },
    {
      name: "description",
      content:
        "Convenient summaries of Kenyan National Assembly and Senate proceedings, making legislative information more accessible and digestible.",
    },
    { name: "author", content: "Bunge Bits" },
    {
      property: "og:title",
      content: "Bunge Bits - Legislative Summaries for the Parliament of Kenya",
    },
    {
      property: "og:description",
      content:
        "Convenient summaries of Kenyan National Assembly and Senate proceedings, making legislative information more accessible and digestible.",
    },
    { property: "og:type", content: "website" },
    {
      property: "og:image",
      content: `${origin}/bunge-bits/logo_1024x1024.png`,
    },
    { name: "twitter:card", content: "summary_large_image" },
    { name: "twitter:site", content: "@bungebits" },
    {
      name: "twitter:image",
      content: `${origin}/bunge-bits/logo_1024x1024.png`,
    },
  ];
};

export function Layout({ children }: { children: React.ReactNode }) {
  const data = useLoaderData<typeof loader>();
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
      </head>
      <body>
        <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50">
          <Header />
          <NavigationProgress />
          {children}
          <Footer />
        </div>
        <ScrollRestoration />
        <Scripts />
        {data !== undefined &&
          data.env?.PLAUSIBLE_BASE_URL &&
          data.env?.PLAUSIBLE_DOMAIN &&
          process.env.NODE_ENV === "production" && (
            <script
              defer
              data-domain={data.env.PLAUSIBLE_DOMAIN}
              src={`${data.env.PLAUSIBLE_BASE_URL}/js/script.js`}
            ></script>
          )}
      </body>
    </html>
  );
}

export function ErrorBoundary() {
  return <ErrorPage error={useRouteError()} />;
}

export default function App() {
  return <Outlet />;
}
