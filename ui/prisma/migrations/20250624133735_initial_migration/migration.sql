-- CreateTable
CREATE TABLE "search_queries" (
    "id" SERIAL NOT NULL,
    "query" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "search_queries_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "streams" (
    "video_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "view_count" TEXT NOT NULL,
    "stream_timestamp" TIMESTAMPTZ(6) NOT NULL,
    "duration" TEXT NOT NULL,
    "summary_md" TEXT,
    "timestamp_md" TEXT,
    "is_published" BOOLEAN NOT NULL DEFAULT false,
    "search_vector" tsvector,
    "house" TEXT,

    CONSTRAINT "streams_pkey" PRIMARY KEY ("video_id")
);

-- CreateIndex
CREATE INDEX "idx_search_queries_created_at" ON "search_queries"("created_at");

-- CreateIndex
CREATE INDEX "streams_search_vector_idx" ON "streams" USING GIN ("search_vector");
