import { Card, CardContent, CardHeader } from "./ui/card";

export default function DetailPageSekeleton() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-green-50 to-blue-50">
      <main className="container mx-auto px-4 py-8 max-w-4xl">
        <div className="mb-6">
          <div className="w-40 h-9 bg-gray-200 rounded-md animate-pulse" />
        </div>

        <Card className="bg-white/90 backdrop-blur-sm border-0 shadow-lg">
          <CardHeader className="pb-6">
            <div className="flex flex-col md:flex-row md:items-start md:justify-between gap-4 mb-4">
              <div className="w-20 h-6 bg-gray-200 rounded-md animate-pulse" />
              <div className="flex gap-4">
                <div className="w-32 h-4 bg-gray-200 rounded animate-pulse" />
                <div className="w-24 h-4 bg-gray-200 rounded animate-pulse" />
              </div>
            </div>

            <div className="w-full h-6 bg-gray-300 rounded-md animate-pulse mb-2" />
            <div className="w-3/4 h-6 bg-gray-200 rounded-md animate-pulse" />
          </CardHeader>

          <div className="p-4">
            <div className="rounded-md overflow-hidden aspect-video w-full bg-gray-200 animate-pulse" />
          </div>

          <CardContent className="space-y-4">
            {Array.from({ length: 5 }).map((_, i) => (
              <div key={i} className="h-4 w-full bg-gray-200 rounded animate-pulse" />
            ))}
            <div className="h-4 w-2/3 bg-gray-200 rounded animate-pulse" />
          </CardContent>
        </Card>
      </main>
    </div>
  );
}
