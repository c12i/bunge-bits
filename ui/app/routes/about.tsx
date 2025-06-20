import { MetaFunction } from "@remix-run/node";

export const meta: MetaFunction = () => [
  { title: "About | Bunge Bits" },
  { name: "description", content: "Learn more about the Bunge Bits project" },
];

export default function AboutPage() {
  return (
    <div className="max-w-3xl mx-auto px-4 py-12 space-y-6">
      <h1 className="text-3xl font-bold">About Bunge Bits</h1>

      <p>
        <strong>Bunge Bits</strong> is a civic-tech project that uses AI to generate
        human-readable summaries of Kenya’s National Assembly and Senate proceedings. The
        goal is to make parliamentary activity more accessible to all Kenyans—especially
        those who don’t have the time or capacity to sit through multi-hour livestreams.
      </p>

      <h2 className="text-xl font-semibold mt-6">How It Works</h2>
      <p>
        At the core of Bunge Bits is a fully automated pipeline that:
        <ol className="list-decimal list-inside mt-2 space-y-1">
          <li>Scrapes livestreams from official YouTube channels</li>
          <li>Downloads and transcribes audio using OpenAI Whisper</li>
          <li>Generates structured summaries with GPT-4o</li>
        </ol>
      </p>

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
        features for exploring trends over time.
      </p>
      <p>
        If you're a developer, designer, journalist, or researcher interested in civic
        tech, I’d love to collaborate and take this further.
      </p>

      <h2 className="text-xl font-semibold">Bug Reports and Feature Requests</h2>
      <p>
        Found a bug or have an idea to improve the platform? Feel free to open an issue or
        feature request on the project’s GitHub repository:{" "}
        <a
          href="https://github.com/c12i/bunge-bits/issues"
          target="_blank"
          rel="noopener noreferrer"
          className="underline hover:text-primary"
        >
          github.com/c12i/bunge-bits/issues
        </a>
        .
      </p>

      <h2 className="text-xl font-semibold mt-6">Support the Project</h2>
      <p>
        Running this platform incurs real costs. API usage for Whisper and GPT-4o, cloud
        hosting, and database storage. If you’d like to help keep it alive and growing,
        your support would mean a lot. You can do so here:{" "}
        <a
          href="https://www.buymeacoffee.com/c12i"
          target="_blank"
          rel="noopener noreferrer"
          className="underline hover:text-primary"
        >
          buymeacoffee.com/c12i
        </a>
        .
      </p>

      <p className="text-sm text-muted-foreground mt-4">
        <a
          href="https://github.com/c12i/bunge-bits"
          target="_blank"
          rel="noopener noreferrer"
          className="underline"
        >
          GitHub
        </a>{" "}
        | Contact: <code>hello[at]c12i.xyz</code>
      </p>
    </div>
  );
}
