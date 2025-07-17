import { MetaFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { format } from "date-fns";

import { useHasHydrated } from "~/lib/hooks";

export const meta: MetaFunction = () => [
  { title: "About | Bunge Bits" },
  { name: "description", content: "Learn more about the Bunge Bits project" },
];

type BackendStatus = {
  healthy: boolean;
  next_tick: Date;
};

export async function loader() {
  try {
    const res = await fetch("https://bungebits-status.c12i.xyz/status");
    const data: BackendStatus = await res.json();

    return Response.json(
      {
        healthy: data.healthy ?? false,
        next_tick: data.next_tick ?? null,
      },
      {
        headers: {
          "Cache-Control": "public, max-age=1800, stale-while-revalidate=3600",
        },
      }
    );
  } catch {
    return Response.json(
      {
        healthy: false,
        next_tick: null,
      },
      {
        headers: {
          "Cache-Control": "public, max-age=1800, stale-while-revalidate=3600",
        },
      }
    );
  }
}

export default function AboutPage() {
  const { healthy, next_tick } = useLoaderData<typeof loader>();
  const hasHydrated = useHasHydrated();

  if (!hasHydrated) {
    return (
      <div className="flex flex-col items-center py-12">
        <div className="w-6 h-6 border-4 border-destructive border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  return (
    <div className="max-w-3xl mx-auto px-4 py-12 space-y-6">
      <h1 className="text-3xl font-bold">About Bunge Bits</h1>

      <p>
        <strong>Bunge Bits</strong> is a civic-tech project that uses AI to generate
        human-readable summaries of Kenya’s National Assembly and Senate proceedings. The
        goal is to make parliamentary activity more accessible to all Kenyans, especially
        those who don’t have the time or capacity to sit through multi-hour livestreams.
      </p>

      <h2 className="text-xl font-semibold mt-6">How It Works</h2>
      <div>
        <p>At the core of Bunge Bits is a fully automated pipeline that:</p>
        <ol className="list-decimal list-inside mt-2 space-y-1">
          <li>
            Periodically scrapes archived livestreams from the official Parliament of
            Kenya YouTube channel
          </li>
          <li>Downloads and transcribes audio using OpenAI Whisper</li>
          <li>Generates structured summaries with GPT-4o</li>
        </ol>
      </div>

      <h2 className="text-xl font-semibold mt-6">Tech Stack</h2>
      <p>
        The system is written in <strong>Rust</strong>, with a <strong>Remix.js</strong>{" "}
        frontend and a <strong>PostgreSQL</strong> database. Cloud infra is containerized
        and runs on a self-hosted VPS.
      </p>

      <h2 className="text-xl font-semibold mt-6">Why This Project Matters</h2>
      <p>
        Kenya’s democracy thrives when citizens are informed. Yet most people are
        disconnected from the day-to-day workings of Parliament. Bunge Bits tries to
        bridge that gap, by making civic information simple, digestible, and shareable.
      </p>

      <h2 className="text-xl font-semibold mt-6">Future Direction</h2>
      <p>
        This project is just getting started. I plan to expand support for Swahili
        translations, integrate named-entity recognition to track bills and MPs, and build
        features for exploring trends over time via Hansard reports and historical
        summaries.
      </p>
      <p>
        If you're a developer, designer, journalist, or researcher interested in civic
        tech, I’d love to collaborate and take this further.
      </p>

      <h2 className="text-xl font-semibold mt-6">Support the Project</h2>
      <p>
        Running this platform incurs real costs. API usage for Whisper and GPT-4o, cloud
        hosting, and database storage. If you’d like to help keep it alive and growing,
        your support would mean a lot. You can do so here:{" "}
        <a
          href="https://support-bungebits.c12i.xyz"
          target="_blank"
          rel="noopener noreferrer"
          className="underline hover:text-primary"
        >
          support-bungebits.c12i.xyz
        </a>
      </p>

      <p className="text-sm text-muted-foreground mt-4">
        <a
          href="https://c12i.xyz"
          target="_blank"
          rel="noopener noreferrer"
          className="underline"
        >
          Collins Muriuki
        </a>{" "}
        | Contact: <code>hello[at]c12i[dot]xyz</code>
      </p>

      <div className="text-sm text-muted-foreground bg-muted/40 rounded-md px-4 py-3 mt-6 text-center">
        <p className="mb-1 flex justify-center items-center gap-2">
          <span
            className={`h-2 w-2 rounded-full ${healthy ? "bg-green-500" : "bg-red-500"}`}
          />
          Backend: {healthy ? "Healthy" : "Unavailable"}
        </p>
        {next_tick && (
          <p className="text-xs">
            Next scheduled update:{" "}
            <span className="font-mono">
              {format(new Date(next_tick), "MMM d, yyyy HH:mm")} (Nairobi Time)
            </span>
          </p>
        )}
      </div>
    </div>
  );
}
