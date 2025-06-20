import { json, LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import { Link, useLoaderData } from "@remix-run/react";
import {
  ArrowLeft,
  Calendar,
  Clock,
  ExternalLink,
  Play,
  Users,
} from "lucide-react";
import snarkdown from "snarkdown";
import ReactMarkdown from "react-markdown";

import Header from "~/components/header";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Separator } from "~/components/ui/separator";
import { formatDate, formatDuration } from "~/lib/utils";

import { PrismaClient } from "@prisma-app/client";
const prisma = new PrismaClient();

export async function loader({ params }: LoaderFunctionArgs) {
  const { videoId } = params;

  if (!videoId) {
    throw new Response("Missing video ID", { status: 400 });
  }

  const stream = await prisma.streams.findUnique({
    where: { video_id: videoId },
  });

  if (!stream) {
    throw new Response("Not Found", { status: 404 });
  }

  return json({ stream });
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
  const cleanedMarkdown = rawMarkdown.replace(/\\n/g, "\n"); // key line

  return (
    <div className="min-h-screen bg-gradient-to-br from-green-50 to-blue-50">
      <Header />

      <main className="container mx-auto px-4 py-8 max-w-4xl">
        <div className="mb-6">
          <Link to="/summaries">
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
                Parliament
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
                <div className="flex items-center">
                  <Users className="w-4 h-4 mr-2" />
                  {parseInt(stream.view_count).toLocaleString()} views
                </div>
              </div>
            </div>

            <CardTitle className="text-2xl md:text-3xl leading-tight text-gray-900 mb-4">
              {stream.title}
            </CardTitle>
          </CardHeader>

          <CardContent className="space-y-8">
            <div>
              <h2 className="text-xl font-semibold text-gray-900 mb-4">
                Summary
              </h2>
              <div className="markdown">
                <ReactMarkdown>{cleanedMarkdown}</ReactMarkdown>
              </div>
            </div>

            <Separator />

            <div className="flex flex-col sm:flex-row gap-4">
              <Button
                asChild
                className="bg-green-600 hover:bg-green-700 text-white flex-1"
              >
                <a
                  href={`https://youtube.com/watch?v=${stream.video_id}`}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <Play className="w-4 h-4 mr-2" />
                  Watch Full Session
                  <ExternalLink className="w-4 h-4 ml-2" />
                </a>
              </Button>

              <Button variant="outline" asChild className="flex-1">
                <Link to="/summaries">View More Summaries</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </main>
    </div>
  );
}
