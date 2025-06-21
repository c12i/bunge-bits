import { PrismaClient } from "@prisma-app/client";

import { toRssFeed } from "~/lib/utils";

const prisma = new PrismaClient();

export const loader = async () => {
  const summaries = await prisma.streams.findMany({
    where: { is_published: true },
    orderBy: { stream_timestamp: "desc" },
    take: 20,
  });

  const feed = toRssFeed({
    title: "Bunge Bits – Parliamentary Summaries",
    description: "Bite-sized summaries of Kenyan parliamentary proceedings.",
    baseUrl: "https://bungebits.ke",
    items: summaries.map((s) => ({
      title: s.title,
      slug: `/summaries/${s.video_id}`,
      date: s.stream_timestamp,
    })),
  });

  return new Response(feed, {
    status: 200,
    headers: {
      "Content-Type": "application/rss+xml",
    },
  });
};
