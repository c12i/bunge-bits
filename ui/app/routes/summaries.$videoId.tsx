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

import Header from "~/components/header";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Separator } from "~/components/ui/separator";
import { mockStreams } from "~/data/mock-data";

export async function loader({ params }: LoaderFunctionArgs) {
  const stream = mockStreams.find((s) => s.videoId === params.videoId);

  if (!stream) {
    throw new Response("Not Found", { status: 404 });
  }

  return json({ stream });
}

export const meta: MetaFunction = ({}) => {
  return [
    { title: `Summary – Parliamentary Session` },
    { name: "description", content: "Session summary for videoId..." },
  ];
};

export default function StreamSummary() {
  const { stream } = useLoaderData<typeof loader>();

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString("en-GB", {
      year: "numeric",
      month: "long",
      day: "numeric",
      weekday: "long",
    });
  };

  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

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
              <Badge
                variant={
                  stream.chamber === "National Assembly"
                    ? "default"
                    : "secondary"
                }
                className="w-fit"
              >
                {stream.chamber}
              </Badge>
              <div className="flex flex-wrap items-center gap-4 text-sm text-gray-600">
                <div className="flex items-center">
                  <Calendar className="w-4 h-4 mr-2" />
                  {formatDate(stream.date)}
                </div>
                <div className="flex items-center">
                  <Clock className="w-4 h-4 mr-2" />
                  {formatDuration(stream.duration)}
                </div>
                <div className="flex items-center">
                  <Users className="w-4 h-4 mr-2" />
                  {stream.viewCount.toLocaleString()} views
                </div>
              </div>
            </div>

            <CardTitle className="text-2xl md:text-3xl leading-tight text-gray-900 mb-4">
              {stream.title}
            </CardTitle>

            <div className="flex flex-wrap gap-2">
              {stream.keyTopics?.map((topic: string, index: number) => (
                <Badge key={index} variant="outline" className="text-sm">
                  {topic}
                </Badge>
              ))}
            </div>
          </CardHeader>

          <CardContent className="space-y-8">
            <div>
              <h2 className="text-xl font-semibold text-gray-900 mb-4">
                Executive Summary
              </h2>
              <p className="text-gray-700 leading-relaxed text-lg">
                {stream.summary}
              </p>
            </div>

            <Separator />

            <div>
              <h2 className="text-xl font-semibold text-gray-900 mb-4">
                Detailed Summary
              </h2>
              <div className="prose prose-gray max-w-none">
                <p className="text-gray-700 leading-relaxed mb-4">
                  {stream.detailedSummary}
                </p>
              </div>
            </div>

            <Separator />

            <div>
              <h2 className="text-xl font-semibold text-gray-900 mb-4">
                Key Moments
              </h2>
              <div className="bg-gray-50 rounded-lg p-6">
                <div className="space-y-4">
                  {stream.moments?.map((moment: any, index: number) => (
                    <div key={index} className="flex items-start gap-4">
                      <div className="bg-red-800 text-white px-3 py-1 rounded-md text-sm font-mono min-w-fit">
                        {moment.timestamp}
                      </div>
                      <p className="text-gray-700 leading-relaxed flex-1">
                        {moment.description}
                      </p>
                    </div>
                  ))}
                </div>
              </div>
            </div>

            <Separator />

            <div>
              <h2 className="text-xl font-semibold text-gray-900 mb-4">
                Key Points Discussed
              </h2>
              <ul className="space-y-3">
                {stream.keyPoints?.map((point: string, index: number) => (
                  <li key={index} className="flex items-start">
                    <div className="w-2 h-2 bg-green-600 rounded-full mt-2 mr-3 flex-shrink-0"></div>
                    <p className="text-gray-700 leading-relaxed">{point}</p>
                  </li>
                ))}
              </ul>
            </div>

            <Separator />

            <div className="flex flex-col sm:flex-row gap-4">
              <Button
                asChild
                className="bg-green-600 hover:bg-green-700 text-white flex-1"
              >
                <a
                  href={`https://youtube.com/watch?v=${stream.videoId}`}
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
