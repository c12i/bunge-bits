import "./tailwind.css";

import type { LinksFunction, LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  useLoaderData,
} from "@remix-run/react";

import Footer from "./components/footer";
import Header from "./components/header";

export const links: LinksFunction = () => [
  {
    rel: "alternate",
    type: "application/rss+xml",
    title: "Bunge Bits RSS Feed",
    href: "/summaries/rss.xml",
  },
];

export async function loader({ request }: LoaderFunctionArgs) {
  const origin = new URL(request.url).origin;

  return Response.json({
    origin,
    env: {
      PLAUSIBLE_BASE_URL: process.env.PUBLIC_PLAUSIBLE_BASE_URL,
      PLAUSIBLE_DOMAIN: process.env.PUBLIC_PLAUSIBLE_DOMAIN,
    },
  });
}

export const meta: MetaFunction<typeof loader> = ({ data }) => {
  const origin = data!.origin;

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
  const { env } = useLoaderData<typeof loader>();
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
      </head>
      <body>
        <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50">
          <Header />
          {children}
          <Footer />
        </div>
        <ScrollRestoration />
        <Scripts />
        {env?.PLAUSIBLE_BASE_URL &&
          env?.PLAUSIBLE_DOMAIN &&
          process.env.NODE_ENV === "production" && (
            <script
              defer
              data-domain={env.PLAUSIBLE_DOMAIN}
              src={`${env.PLAUSIBLE_BASE_URL}/js/script.js`}
            ></script>
          )}
      </body>
    </html>
  );
}

export default function App() {
  return <Outlet />;
}
