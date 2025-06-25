import { PrismaClient } from "@prisma-app/client";
import { LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import { Link, useLoaderData, useLocation } from "@remix-run/react";
import { ArrowLeft, Calendar, Clock } from "lucide-react";
import ReactMarkdown from "react-markdown";

import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { highlightChildren, highlightText } from "~/lib/text-highlight";
import { formatDate, formatDuration } from "~/lib/utils";

const prisma = new PrismaClient();

export async function loader({ params }: LoaderFunctionArgs) {
  const { videoId } = params;

  if (!videoId) {
    throw new Response("Missing video ID", { status: 400 });
  }

  try {
    const stream = await prisma.streams.findUnique({
      where: { video_id: videoId },
    });

    if (!stream) {
      throw new Response("Not Found", { status: 404 });
    }

    return Response.json(
      { stream },
      {
        headers: {
          "Cache-Control":
            "public, max-age=600, s-maxage=3600, stale-while-revalidate=86400",
        },
      }
    );
  } catch (err) {
    console.error("DB fetch failed:", err);
    throw new Response("Internal Server Error", { status: 500 });
  }
}

export const meta: MetaFunction<typeof loader> = ({ data }) => {
  if (!data) {
    return [
      { title: "Summary – Parliamentary Session" },
      {
        name: "description",
        content: "Session summary for a legislative stream.",
      },
    ];
  }

  return [
    { title: `Summary – ${data.stream.title}` },
    {
      name: "description",
      content:
        data.stream.summary_md?.slice(0, 150).replace(/\n/g, " ") ||
        "Session summary for a legislative stream.",
    },
  ];
};

export default function StreamSummary() {
  const { stream } = useLoaderData<typeof loader>();
  const rawMarkdown = stream.summary_md || "";
  const cleanedMarkdown = rawMarkdown.replace(/\\n/g, "\n");

  const location = useLocation();
  const backSearch = location.search || "";

  const query = new URLSearchParams(location.search).get("q") || "";
  const queryTerm = query?.toLowerCase().trim() || "";

  return (
    <div className="min-h-screen bg-gradient-to-br from-green-50 to-blue-50">
      <main className="container mx-auto px-4 py-8 max-w-4xl">
        <div className="mb-6">
          <Link to={{ pathname: `/summaries`, search: backSearch }}>
            <Button variant="ghost" className="mb-4 hover:bg-white/50">
              <ArrowLeft className="w-4 h-4 mr-2" />
              Back to Summaries
            </Button>
          </Link>
        </div>

        <Card className="bg-white/90 backdrop-blur-sm border-0 shadow-lg">
          <CardHeader className="pb-6">
            <div className="flex flex-col md:flex-row md:items-start md:justify-between gap-4 mb-4">
              <Badge variant="default" className="w-fit">
                {stream.house}
              </Badge>
              <div className="flex flex-wrap items-center gap-4 text-sm text-gray-600">
                <div className="flex items-center">
                  <Calendar className="w-4 h-4 mr-2" />
                  {formatDate(stream.stream_timestamp)}
                </div>
                <div className="flex items-center">
                  <Clock className="w-4 h-4 mr-2" />
                  {formatDuration(stream.duration)}
                </div>
              </div>
            </div>

            <CardTitle className="text-2xl md:text-3xl leading-tight text-gray-900 mb-4">
              {highlightText(stream.title, queryTerm)}
            </CardTitle>
          </CardHeader>

          <div className="p-4">
            <div className="rounded-md overflow-hidden aspect-video w-full">
              <iframe
                src={`https://www.youtube.com/embed/${stream.video_id}`}
                className="w-full h-full"
                title="YouTube video player"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                allowFullScreen
                loading="lazy"
              />
            </div>
          </div>

          <CardContent className="space-y-8">
            <div>
              <div className="markdown">
                <ReactMarkdown
                  components={{
                    p({ children }) {
                      return <p>{highlightChildren(children, queryTerm)}</p>;
                    },
                    h1({ children }) {
                      return <h1>{highlightChildren(children, queryTerm)}</h1>;
                    },
                    h2({ children }) {
                      return <h2>{highlightChildren(children, queryTerm)}</h2>;
                    },
                    li({ children }) {
                      return <li>{highlightChildren(children, queryTerm)}</li>;
                    },
                    blockquote: ({ children }) => (
                      <blockquote>{highlightChildren(children, queryTerm)}</blockquote>
                    ),
                    strong: ({ children }) => (
                      <strong>{highlightChildren(children, queryTerm)}</strong>
                    ),
                    em: ({ children }) => (
                      <em>{highlightChildren(children, queryTerm)}</em>
                    ),
                    span: ({ children }) => (
                      <span>{highlightChildren(children, queryTerm)}</span>
                    ),
                  }}
                >
                  {cleanedMarkdown}
                </ReactMarkdown>
              </div>
            </div>
          </CardContent>
        </Card>
      </main>
    </div>
  );
}
