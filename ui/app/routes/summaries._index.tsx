import { PrismaClient, streams } from "@prisma-app/client";
import { HeadersFunction, LoaderFunctionArgs } from "@remix-run/node";
import {
  Await,
  Link,
  useLoaderData,
  useLocation,
  useSearchParams,
  useSubmit,
} from "@remix-run/react";
import { Calendar, ChevronLeft, ChevronRight, Clock, Search } from "lucide-react";
import { Suspense, useEffect, useRef, useState } from "react";
import removeMarkdown from "remove-markdown";

import SummariesSkeleton from "~/components/list-page-skeleton";
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
import { highlightText } from "~/lib/text-highlight";
import { formatDate, formatDuration, titleCase } from "~/lib/utils";

const prisma = new PrismaClient();
const PAGE_SIZE = 9;

export async function loader({ request }: LoaderFunctionArgs) {
  const url = new URL(request.url);

  const query = url.searchParams.get("q")?.trim() || "";
  const page = Math.max(parseInt(url.searchParams.get("page") || "1", 10), 1);

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
            house,
            summary_md
          FROM streams 
          WHERE is_published = true
            AND search_vector @@ plainto_tsquery('english', $1)
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
          WHERE is_published = true
            AND search_vector @@ plainto_tsquery('english', $1)
          `,
          query
        ),
      ]);

      return Response.json({ streams, total: countResult[0].count, page, query });
    } catch (error) {
      console.error("Search error:", error);
      const { streams, total } = await fallbackSearch(query, page);
      return Response.json({ streams, total, page, query });
    }
  }

  // Fallback for no query
  const [streams, total] = await Promise.all([
    prisma.streams.findMany({
      where: { is_published: true },
      orderBy: { stream_timestamp: "desc" },
      skip: (page - 1) * PAGE_SIZE,
      take: PAGE_SIZE,
    }),
    prisma.streams.count({
      where: { is_published: true },
    }),
  ]);

  return Response.json({ streams, total, page, query: null });
}

async function fallbackSearch(query: string, page: number) {
  const [streams, count] = await Promise.all([
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

  return { streams, total: count };
}

export const headers: HeadersFunction = () => ({
  "Cache-Control": "public, max-age=600, s-maxage=3600, stale-while-revalidate=86400",
});

export default function Index() {
  const { streams, total, query } = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();

  const page = Number(searchParams.get("page") || 1);
  const pageCount = Math.ceil(total / PAGE_SIZE);

  const { inputValue, handleInputChange, handleClearSearch } = useSearch();
  const queryTerms = query?.toLowerCase().trim() || "";

  return (
    <Suspense fallback={<SummariesSkeleton />}>
      <Await resolve={streams}>
        <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50">
          <main className="container mx-auto px-4 py-8 max-w-6xl">
            <div className="text-center mb-12">
              <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">
                Bunge Bits
              </h1>
              <p className="text-xl text-gray-600 mb-6 max-w-3xl mx-auto">
                Convenient summaries of Kenyan National Assembly and Senate proceedings,
                making legislative information more accessible and digestible. Powered by
                AI.
              </p>

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

            <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
              {streams.map((stream: streams) => (
                <StreamSummariesCard
                  key={stream.video_id}
                  stream={stream}
                  queryTerms={queryTerms}
                />
              ))}
            </div>

            {streams.length === 0 && (
              <div className="text-center">
                <p className="text-gray-500 text-lg">
                  No summaries found for "{query ? query : "this page"}".
                </p>
                {query && (
                  <button
                    onClick={handleClearSearch}
                    className="text-red-800 hover:underline mt-2 inline-block cursor-pointer"
                  >
                    View all summaries
                  </button>
                )}
              </div>
            )}

            {streams.length > 0 && (
              <Pagination currentPage={page} totalPages={pageCount} />
            )}
          </main>
        </div>
      </Await>
    </Suspense>
  );
}

export function HydrationFallback() {
  return <SummariesSkeleton />;
}

type StreamSummariesCardProps = {
  stream: streams;
  queryTerms: string;
};

export function StreamSummariesCard({ stream, queryTerms }: StreamSummariesCardProps) {
  return (
    <Card
      key={stream.video_id}
      className="group hover:shadow-lg transition-all duration-300 hover:-translate-y-1 bg-white/80 backdrop-blur-sm border-0 shadow-md"
    >
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between gap-2 mb-2">
          <Badge
            variant="default"
            className={`text-xs ${
              stream.house === "senate"
                ? "bg-accent hover:bg-accent text-accent-foreground"
                : stream.house === "national assembly"
                  ? "bg-primary hover:bg-primary text-primary-foreground"
                  : stream.house === "all"
                    ? "bg-secondary hover:bg-secondary text-secondary-foreground"
                    : "bg-muted hover:bg-muted text-muted-foreground"
            }`}
          >
            {stream.house}
          </Badge>
          <div className="flex items-center text-xs text-gray-500">
            <Calendar className="w-3 h-3 mr-1" />
            {formatDate(stream.stream_timestamp)}
          </div>
        </div>
        <CardTitle className="text-lg leading-tight group-hover:text-red-800 transition-colors">
          {highlightText(titleCase(stream.title), queryTerms)}
        </CardTitle>
        <div className="flex items-center gap-4 text-xs text-gray-500">
          <div className="flex items-center">
            <Clock className="w-3 h-3 mr-1" />
            {formatDuration(stream.duration)}
          </div>
        </div>
      </CardHeader>
      <CardContent className="pt-0">
        <CardDescription className="line-clamp-3 mb-4 text-sm leading-relaxed">
          {highlightText(getPreviewText(stream.summary_md!), queryTerms)}
        </CardDescription>
        <Link
          to={{
            pathname: `/summaries/${stream.video_id}`,
            search: location.search,
          }}
        >
          <Button className="w-full bg-red-800 hover:bg-red-900 text-white">
            Read Full Summary
          </Button>
        </Link>
      </CardContent>
    </Card>
  );
}

type PaginationProps = {
  currentPage: number;
  totalPages: number;
};

export function Pagination({ currentPage, totalPages }: PaginationProps) {
  const [searchParams] = useSearchParams();
  const location = useLocation();
  const pathname = location.pathname;

  const [page, setPage] = useState(currentPage);
  const [isOverridden, setIsOverridden] = useState(false);

  useEffect(() => {
    setPage(currentPage);
    setIsOverridden(false);
  }, [currentPage]);

  const goToPage = (targetPage: number) => {
    const validPage = Math.max(1, Math.min(targetPage, totalPages));
    const newParams = new URLSearchParams({
      ...Object.fromEntries(searchParams),
      page: String(validPage),
    });
    window.location.href = `${pathname}?${newParams.toString()}`;
  };

  return (
    <div className="flex justify-center mt-8 items-center gap-2">
      {currentPage > 1 && (
        <Button
          variant="outline"
          size="icon"
          disabled={currentPage === 1}
          onClick={() => {
            if (isOverridden) {
              goToPage(page);
            } else {
              goToPage(currentPage - 1);
            }
          }}
        >
          <ChevronLeft className="h-4 w-4" />
        </Button>
      )}

      <form
        onSubmit={(e) => {
          e.preventDefault();
          goToPage(page);
        }}
        className="flex items-center gap-2"
      >
        <input
          name="page"
          type="number"
          min={1}
          max={totalPages}
          value={page}
          onChange={(e) => {
            setPage(Number(e.target.value));
            setIsOverridden(true);
          }}
          className="w-10 h-9 text-center border rounded-md px-2 text-sm
               [appearance:textfield]
               [&::-webkit-outer-spin-button]:appearance-none
               [&::-webkit-inner-spin-button]:appearance-none"
        />
        <span className="text-sm text-gray-700">of {totalPages}</span>
      </form>

      {currentPage < totalPages && (
        <Button
          variant="outline"
          size="icon"
          disabled={currentPage === totalPages}
          onClick={() => {
            if (isOverridden) {
              goToPage(page);
            } else {
              goToPage(currentPage + 1);
            }
          }}
        >
          <ChevronRight className="h-4 w-4" />
        </Button>
      )}
    </div>
  );
}

function useSearch() {
  const [searchParams] = useSearchParams();
  const submit = useSubmit();
  const searchTerm = searchParams.get("q") || "";
  const [inputValue, setInputValue] = useState(searchTerm);
  const debouncedSearchTerm = useDebounce(inputValue, 300);
  const hasMountedRef = useRef(false);

  useEffect(() => {
    const newSearchParams = new URLSearchParams();

    if (debouncedSearchTerm.trim()) {
      newSearchParams.set("q", debouncedSearchTerm.trim());
    }
    newSearchParams.set("page", "1");

    // only submit if the debounced value is different from current URL param
    if (debouncedSearchTerm.trim() !== searchTerm) {
      submit(newSearchParams, { method: "get" });
    }
  }, [debouncedSearchTerm, submit, searchTerm]);

  // update input value when URL changes (e.g., from navigation)
  useEffect(() => {
    if (!hasMountedRef.current) {
      setInputValue(searchTerm);
      hasMountedRef.current = true;
    }
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

function getPreviewText(summaryMd: string): string {
  const bodyOnly = summaryMd.split("\n").slice(1).join("\n").trim();
  const cleaned = removeMarkdown(bodyOnly || summaryMd)
    .replace(/\\n/g, " ")
    .replace(/\s+/g, " ")
    .trim();

  return cleaned || "No Preview Available";
}
