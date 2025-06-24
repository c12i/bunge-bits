import { PrismaClient } from "@prisma-app/client";

const prisma = new PrismaClient();

function deriveHouse(title: string): string {
  const lower = title.toLowerCase();
  if (lower.includes("national assembly") && lower.includes("senate")) return "all";
  if (lower.includes("national assembly")) return "national assembly";
  if (lower.includes("senate")) return "senate";
  return "unspecified";
}

async function main() {
  const streams = [
    {
      video_id: "rCXO3Yc5bYc",
      title: "TUESDAY 24TH JUNE , 2025 | AFTERNOON SESSION",
      view_count: "8200",
      stream_timestamp: new Date("2025-06-24T14:00:00Z"),
      duration: "1h22m",
      summary_md: "Afternoon session covering multiple bills.",
      timestamp_md: "- 00:01 Opening\n- 00:20 Debate on Finance Bill",
      is_published: true,
    },
    {
      video_id: "CEsTRpeOGkg",
      title: "THURSDAY 19TH JUNE , 2025 | AFTERNOON SESSION",
      view_count: "3300",
      stream_timestamp: new Date("2025-06-19T14:00:00Z"),
      duration: "1h20m",
      summary_md: "Highlighted speeches and questions.",
      timestamp_md: "- 00:05 Roll-call\n- 00:25 Motions",
      is_published: true,
    },
    {
      video_id: "WogLNxA9Uv8",
      title: "The Senate Plenary, Tuesday 10th June 2025. Afternoon Session",
      view_count: "9200",
      stream_timestamp: new Date("2025-06-10T14:00:00Z"),
      duration: "44m58s",
      summary_md: "Senate debate on county legislation funding.",
      timestamp_md: "- 00:02 Opening\n- 00:15 Committee report",
      is_published: true,
    },
  ];

  for (const s of streams) {
    await prisma.streams.upsert({
      where: { video_id: s.video_id },
      update: {},
      create: {
        ...s,
        house: deriveHouse(s.title),
      },
    });
  }

  console.log(`✅ Seeded ${streams.length} Parliament videos`);
}

main()
  .catch((e) => {
    console.error("❌ Error:", e);
    process.exit(1);
  })
  .finally(() => prisma.$disconnect());
