import { PrismaClient } from "@prisma-app/client";
import { HeadersFunction, LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import { Await, Link, useLoaderData, useLocation } from "@remix-run/react";
import { ArrowLeft, Calendar, Clock } from "lucide-react";
import { Suspense } from "react";
import ReactMarkdown from "react-markdown";

import SummarySkeleton from "~/components/detail-page-skeleton";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { highlightText } from "~/lib/text-highlight";
import { formatDate, formatDuration, titleCase } from "~/lib/utils";

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

    return Response.json({ stream });
  } catch (err) {
    console.error("DB fetch failed:", err);
    throw new Response("Internal Server Error", { status: 500 });
  }
}

export const headers: HeadersFunction = () => ({
  "Cache-Control": "public, max-age=600, s-maxage=3600, stale-while-revalidate=86400",
});

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
    <Suspense fallback={<SummarySkeleton />}>
      <Await resolve={stream}>
        <div className="min-h-screen">
          <main className="container mx-auto px-4 py-8 max-w-4xl">
            <div className="mb-6">
              <Link to={{ pathname: `/summaries`, search: backSearch }}>
                <Button variant="ghost" className="mb-4 hover:bg-transparent">
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
                  {highlightText(titleCase(stream.title), queryTerm)}
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

              <CardContent className="space-y-8 lg:px-10">
                <div>
                  <div className="markdown">
                    <ReactMarkdown>{cleanedMarkdown}</ReactMarkdown>
                  </div>
                </div>
              </CardContent>
            </Card>
          </main>
        </div>
      </Await>
    </Suspense>
  );
}

export function HydrationFallback() {
  return <SummarySkeleton />;
}
