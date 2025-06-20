import { Link } from "@remix-run/react";
import { Calendar, Play, Search, Users } from "lucide-react";
import { useState } from "react";

import Header from "~/components/header";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { mockStreams } from "~/data/mock-data";

export default function Index() {
  const [searchTerm, setSearchTerm] = useState("");

  const filteredStreams = mockStreams.filter(
    (stream: any) =>
      stream.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
      stream.summary.toLowerCase().includes(searchTerm.toLowerCase()) ||
      stream.chamber.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString("en-GB", {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  };

  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50">
      <Header />

      <main className="container mx-auto px-4 py-8 max-w-6xl">
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">
            Bunge Bits
          </h1>
          <p className="text-xl text-gray-600 mb-6 max-w-3xl mx-auto">
            Convenient summaries of Kenyan National Assembly and Senate
            proceedings, making legislative information more accessible and
            digestible.
          </p>

          <div className="relative max-w-md mx-auto">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
            <Input
              placeholder="Search summaries..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="pl-10 pr-4 py-2 w-full"
            />
          </div>
        </div>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {filteredStreams.map((stream) => (
            <Card
              key={stream.videoId}
              className="group hover:shadow-lg transition-all duration-300 hover:-translate-y-1 bg-white/80 backdrop-blur-sm border-0 shadow-md"
            >
              <CardHeader className="pb-3">
                <div className="flex items-start justify-between gap-2 mb-2">
                  <Badge
                    variant={
                      stream.chamber === "National Assembly"
                        ? "default"
                        : "secondary"
                    }
                    className="text-xs"
                  >
                    {stream.chamber}
                  </Badge>
                  <div className="flex items-center text-xs text-gray-500">
                    <Calendar className="w-3 h-3 mr-1" />
                    {formatDate(stream.date)}
                  </div>
                </div>
                <CardTitle className="text-lg leading-tight group-hover:text-red-800 transition-colors">
                  {stream.title}
                </CardTitle>
                <div className="flex items-center gap-4 text-xs text-gray-500">
                  <div className="flex items-center">
                    <Play className="w-3 h-3 mr-1" />
                    {formatDuration(stream.duration)}
                  </div>
                  <div className="flex items-center">
                    <Users className="w-3 h-3 mr-1" />
                    {stream.viewCount.toLocaleString()} views
                  </div>
                </div>
              </CardHeader>
              <CardContent className="pt-0">
                <CardDescription className="line-clamp-3 mb-4 text-sm leading-relaxed">
                  {stream.summary}
                </CardDescription>
                <div className="flex flex-wrap gap-1 mb-4">
                  {stream.keyTopics.slice(0, 3).map((topic, index) => (
                    <Badge key={index} variant="outline" className="text-xs">
                      {topic}
                    </Badge>
                  ))}
                  {stream.keyTopics.length > 3 && (
                    <Badge variant="outline" className="text-xs">
                      +{stream.keyTopics.length - 3} more
                    </Badge>
                  )}
                </div>
                <Link to={`/summaries/${stream.videoId}`}>
                  <Button className="w-full bg-red-800 hover:bg-red-900 text-white">
                    Read Full Summary
                  </Button>
                </Link>
              </CardContent>
            </Card>
          ))}
        </div>

        {filteredStreams.length === 0 && (
          <div className="text-center py-12">
            <p className="text-gray-500 text-lg">
              No summaries found matching your search.
            </p>
          </div>
        )}
      </main>
    </div>
  );
}
