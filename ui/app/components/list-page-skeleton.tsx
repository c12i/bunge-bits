import { Card } from "./ui/card";

export default function ListPageSkeleton() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50">
      <main className="container mx-auto px-4 py-8 max-w-6xl">
        <div className="text-center mb-12">
          <div className="w-64 h-10 bg-gray-200 rounded-md mx-auto animate-pulse mb-4" />
          <div className="w-96 h-6 bg-gray-200 rounded-md mx-auto animate-pulse mb-6" />

          <div className="relative max-w-md mx-auto">
            <div className="w-full h-10 bg-gray-200 rounded-md animate-pulse" />
          </div>
        </div>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 9 }).map((_, i) => (
            <Card
              key={i}
              className="bg-white/80 backdrop-blur-sm border-0 shadow-md p-4 animate-pulse"
            >
              <div className="flex justify-between items-center mb-2">
                <div className="w-16 h-4 bg-gray-300 rounded" />
                <div className="w-20 h-4 bg-gray-200 rounded" />
              </div>
              <div className="h-5 w-3/4 bg-gray-300 rounded mb-2" />
              <div className="h-4 w-1/2 bg-gray-200 rounded mb-4" />
              <div className="h-3 w-full bg-gray-200 rounded mb-1" />
              <div className="h-3 w-full bg-gray-200 rounded mb-1" />
              <div className="h-3 w-5/6 bg-gray-200 rounded mb-4" />
              <div className="h-8 w-full bg-gray-300 rounded" />
            </Card>
          ))}
        </div>

        {/* Pagination skeleton */}
        <div className="flex justify-center mt-8 gap-2">
          <div className="w-9 h-9 bg-gray-300 rounded-md animate-pulse" />
          <div className="w-10 h-9 bg-gray-300 rounded-md animate-pulse" />
          <div className="w-9 h-9 bg-gray-300 rounded-md animate-pulse" />
        </div>
      </main>
    </div>
  );
}
