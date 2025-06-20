import "./tailwind.css";

import type { LinksFunction, MetaFunction } from "@remix-run/node";
import { Links, Meta, Outlet, Scripts, ScrollRestoration } from "@remix-run/react";

import Footer from "./components/footer";
import Header from "./components/header";

export const links: LinksFunction = () => [
  { rel: "preconnect", href: "https://fonts.googleapis.com" },
  {
    rel: "preconnect",
    href: "https://fonts.gstatic.com",
    crossOrigin: "anonymous",
  },
];

export const meta: MetaFunction = () => [
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
    content: "Bunge Bits - Legislative Summaries for the Parliement of Kenya",
  },
  {
    property: "og:description",
    content:
      "Convenient summaries of Kenyan National Assembly and Senate proceedings, making legislative information more accessible and digestible.",
  },
  { property: "og:type", content: "website" },
  { name: "twitter:card", content: "summary_large_image" },
  { name: "twitter:site", content: "@bungebits" },
];

export function Layout({ children }: { children: React.ReactNode }) {
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
      </body>
    </html>
  );
}

export default function App() {
  return <Outlet />;
}
