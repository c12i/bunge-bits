import { Github, Rss } from "lucide-react";

export default function Footer() {
  return (
    <footer className="w-full border-t border-muted py-3 text-sm text-muted-foreground flex flex-col items-center">
      <p className="mb-1">&copy; {new Date().getFullYear()} bunge-bits</p>
      <a
        href="https://github.com/c12i/bunge-bits"
        target="_blank"
        rel="noopener noreferrer"
        className="hover:underline inline-flex items-center space-x-1"
      >
        <Github className="w-4 h-4" />
        <span>github.com/c12i/bunge-bits</span>
      </a>

      <a
        href="/summaries/rss.xml"
        className="hover:underline inline-flex items-center space-x-1 mt-1"
      >
        <Rss className="w-4 h-4" />
        <span>Subscribe via RSS</span>
      </a>
    </footer>
  );
}
