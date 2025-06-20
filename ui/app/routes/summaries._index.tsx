import { PrismaClient } from "@prisma-app/client";
import { LoaderFunctionArgs, json } from "@remix-run/node";
import {
  Link,
  useLoaderData,
  useSearchParams,
  useSubmit,
} from "@remix-run/react";
import { Calendar, Play, Search, Users } from "lucide-react";
import { useEffect, useState } from "react";

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
import { useDebounce } from "~/lib/hooks";
import { formatDate, formatDuration } from "~/lib/utils";

const prisma = new PrismaClient();
const PAGE_SIZE = 9;

export async function loader({ request }: LoaderFunctionArgs) {
  const url = new URL(request.url);
  const query = url.searchParams.get("q")?.trim();
  const page = parseInt(url.searchParams.get("page") || "1");

  if (query) {
    try {
      const [streams, countResult] = await Promise.all([
        prisma.$queryRawUnsafe<any[]>(
          `
          SELECT 
            video_id, 
            title, 
            view_count, 
            stream_timestamp, 
            duration, 
            summary_md
          FROM streams 
          WHERE search_vector @@ plainto_tsquery('english', $1)
          ORDER BY stream_timestamp DESC
          OFFSET $2
          LIMIT $3;
        `,
          query,
          (page - 1) * PAGE_SIZE,
          PAGE_SIZE
        ),

        prisma.$queryRawUnsafe<{ count: number }[]>(
          `
          SELECT COUNT(*)::int AS count
          FROM streams 
          WHERE search_vector @@ plainto_tsquery('english', $1)
        `,
          query
        ),
      ]);

      return json({ streams, total: countResult[0].count, page, query });
    } catch (error) {
      console.error("Search error:", error);
      // Fallback to basic search if fts fails
      const [streams, countResult] = await Promise.all([
        prisma.streams.findMany({
          where: {
            OR: [
              { title: { contains: query, mode: "insensitive" } },
              { summary_md: { contains: query, mode: "insensitive" } },
            ],
          },
          orderBy: { stream_timestamp: "desc" },
          skip: (page - 1) * PAGE_SIZE,
          take: PAGE_SIZE,
        }),
        prisma.streams.count({
          where: {
            OR: [
              { title: { contains: query, mode: "insensitive" } },
              { summary_md: { contains: query, mode: "insensitive" } },
            ],
          },
        }),
      ]);

      return json({ streams, total: countResult, page, query });
    }
  }

  // fallback: no query, return all streams paginated
  const [streams, total] = await Promise.all([
    prisma.streams.findMany({
      orderBy: { stream_timestamp: "desc" },
      skip: (page - 1) * PAGE_SIZE,
      take: PAGE_SIZE,
    }),
    prisma.streams.count(),
  ]);

  return json({ streams, total, page, query: null });
}

export default function Index() {
  const { streams, total, query } = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();
  const page = Number(searchParams.get("page") || 1);
  const pageCount = Math.ceil(total / PAGE_SIZE);

  const { inputValue, handleInputChange, handleClearSearch } = useSearch();

  return (
    <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50">
      <main className="container mx-auto px-4 py-8 max-w-6xl">
        {/* Page Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">
            Bunge Bits
          </h1>
          <p className="text-xl text-gray-600 mb-6 max-w-3xl mx-auto">
            Convenient summaries of Kenyan National Assembly and Senate
            proceedings, making legislative information more accessible and
            digestible.
          </p>

          {/* Search Bar */}
          <div className="relative max-w-md mx-auto">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
            <Input
              type="search"
              value={inputValue}
              onChange={(e) => handleInputChange(e.target.value)}
              placeholder="Search summaries..."
              className="pl-10 pr-10 py-2 w-full"
            />
            {inputValue && (
              <button
                type="button"
                onClick={handleClearSearch}
                className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-gray-600"
              >
                ×
              </button>
            )}
          </div>

          {/* Search Results Info */}
          {query && (
            <div className="mt-4 text-sm text-gray-600">
              Showing results for: <strong>"{query}"</strong>
              <button
                onClick={handleClearSearch}
                className="ml-2 text-red-800 hover:underline cursor-pointer"
              >
                Clear search
              </button>
            </div>
          )}
        </div>

        {/* Streams Grid */}
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {streams.map((stream) => (
            <Card
              key={stream.video_id}
              className="group hover:shadow-lg transition-all duration-300 hover:-translate-y-1 bg-white/80 backdrop-blur-sm border-0 shadow-md"
            >
              <CardHeader className="pb-3">
                <div className="flex items-start justify-between gap-2 mb-2">
                  <Badge variant="default" className="text-xs">
                    Parliament
                  </Badge>
                  <div className="flex items-center text-xs text-gray-500">
                    <Calendar className="w-3 h-3 mr-1" />
                    {formatDate(stream.stream_timestamp)}
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
                    {parseInt(stream.view_count).toLocaleString()} views
                  </div>
                </div>
              </CardHeader>
              <CardContent className="pt-0">
                <CardDescription className="line-clamp-3 mb-4 text-sm leading-relaxed">
                  {stream.summary_md || "No summary available."}
                </CardDescription>
                <Link to={`/summaries/${stream.video_id}`}>
                  <Button className="w-full bg-red-800 hover:bg-red-900 text-white">
                    Read Full Summary
                  </Button>
                </Link>
              </CardContent>
            </Card>
          ))}
        </div>

        {/* Pagination */}
        {pageCount > 1 && (
          <div className="flex justify-center mt-8 gap-2">
            <Link
              to={`?${new URLSearchParams({
                ...Object.fromEntries(searchParams),
                page: String(page - 1),
              })}`}
            >
              <Button variant="outline" disabled={page === 1}>
                Prev
              </Button>
            </Link>

            {Array.from({ length: pageCount }).map((_, i) => {
              const newParams = new URLSearchParams({
                ...Object.fromEntries(searchParams),
                page: String(i + 1),
              });
              return (
                <Link key={i} to={`?${newParams.toString()}`}>
                  <Button
                    variant={page === i + 1 ? "default" : "outline"}
                    className="w-10 px-0"
                  >
                    {i + 1}
                  </Button>
                </Link>
              );
            })}

            <Link
              to={`?${new URLSearchParams({
                ...Object.fromEntries(searchParams),
                page: String(page + 1),
              })}`}
            >
              <Button variant="outline" disabled={page === pageCount}>
                Next
              </Button>
            </Link>
          </div>
        )}

        {/* Empty States */}
        {streams.length === 0 && query && (
          <div className="text-center py-12">
            <p className="text-gray-500 text-lg">
              No summaries found for "{query}".
            </p>
            <button
              onClick={handleClearSearch}
              className="text-red-800 hover:underline mt-2 inline-block cursor-pointer"
            >
              View all summaries
            </button>
          </div>
        )}

        {streams.length === 0 && !query && (
          <div className="text-center py-12">
            <p className="text-gray-500 text-lg">
              No summaries found for this page.
            </p>
          </div>
        )}
      </main>
    </div>
  );
}

function useSearch() {
  const [searchParams] = useSearchParams();
  const submit = useSubmit();
  const searchTerm = searchParams.get("q") || "";
  const [inputValue, setInputValue] = useState(searchTerm);
  const debouncedSearchTerm = useDebounce(inputValue, 300);

  useEffect(() => {
    const newSearchParams = new URLSearchParams();

    if (debouncedSearchTerm.trim()) {
      newSearchParams.set("q", debouncedSearchTerm.trim());
    }
    newSearchParams.set("page", "1");

    // Only submit if the debounced value is different from current URL param
    if (debouncedSearchTerm.trim() !== searchTerm) {
      submit(newSearchParams, { method: "get" });
    }
  }, [debouncedSearchTerm, submit, searchTerm]);

  // Update input value when URL changes (e.g., from navigation)
  useEffect(() => {
    setInputValue(searchTerm);
  }, [searchTerm]);

  const handleInputChange = (value: string) => {
    setInputValue(value);
  };

  const handleClearSearch = () => {
    setInputValue("");
    const newSearchParams = new URLSearchParams();
    newSearchParams.set("page", "1");
    submit(newSearchParams, { method: "get" });
  };

  return {
    inputValue,
    handleInputChange,
    handleClearSearch,
  };
}
