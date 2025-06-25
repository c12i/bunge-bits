import { PrismaClient } from "@prisma-app/client";

const prisma = new PrismaClient();

async function main() {
  const streams = [
    {
      video_id: "rCXO3Yc5bYc",
      title: "National Assembly | Tuesday 24th June 2025 | Afternoon Session",
      view_count: "8200",
      stream_timestamp: new Date("2025-06-24T14:00:00Z"),
      duration: "1h22m",
      summary_md: `### 📝 Summary  
This session focused on **public participation and debates surrounding the Finance Bill**. Legislators exchanged views on fiscal measures proposed in the bill, raising concerns on taxation and budget allocation. Key highlights included stakeholder engagement feedback and proposals for amendments to better reflect citizen priorities.`,
      timestamp_md: "- 00:01 Opening remarks\n- 00:18 Finance Bill discussion",
      is_published: true,
    },
    {
      video_id: "CEsTRpeOGkg",
      title: "Senate | Thursday 19th June 2025 | Afternoon Session",
      view_count: "3300",
      stream_timestamp: new Date("2025-06-19T14:00:00Z"),
      duration: "1h20m",
      summary_md: `### 📝 Summary  
This Senate session delved into **county funding proposals**. Senators discussed equitable resource distribution and how devolved units can optimize their budgets. Issues raised included transparency in county expenditures and alignment with national priorities.`,
      timestamp_md: "- 00:05 Roll-call\n- 00:25 Motions on budget allocation",
      is_published: true,
    },
    {
      video_id: "WogLNxA9Uv8",
      title: "Senate | Tuesday 10th June 2025 | Afternoon Session",
      view_count: "9200",
      stream_timestamp: new Date("2025-06-10T14:00:00Z"),
      duration: "44m58s",
      summary_md: `### 📝 Summary  
A concise but crucial sitting where a **committee presented a report on education sector reforms**. The presentation included legislative proposals aimed at improving access, quality, and funding for public education, particularly in underserved regions.`,
      timestamp_md: "- 00:02 Opening\n- 00:15 Committee report on education",
      is_published: true,
    },
    {
      video_id: "4bQfzXvV5TQ",
      title: "National Assembly | Thursday 5th June 2025 | Morning Session",
      view_count: "4100",
      stream_timestamp: new Date("2025-06-05T09:00:00Z"),
      duration: "2h10m",
      summary_md: `### 📝 Summary  
This extended session explored **proposed amendments to existing labor laws**, with members highlighting gaps in labor protections and advocating for better terms for workers. The session concluded with a public address regarding national workforce strategies.`,
      timestamp_md: "- 00:03 Opening\n- 00:20 Discussion on labor bill",
      is_published: true,
    },
    {
      video_id: "Hj2ErV9aH6k",
      title: "Senate | Tuesday 3rd June 2025 | Afternoon Session",
      view_count: "5100",
      stream_timestamp: new Date("2025-06-03T14:00:00Z"),
      duration: "1h05m",
      summary_md: `### 📝 Summary  
Senators reviewed **petitions from various counties regarding water resource management**. Topics included equitable water access, infrastructure development, and the environmental implications of poor regulation.`,
      timestamp_md: "- 00:02 Welcome\n- 00:10 Petition on water rights",
      is_published: true,
    },
    {
      video_id: "x9Jw3yGFeR0",
      title: "National Assembly | Tuesday 27th May 2025 | Morning Session",
      view_count: "6000",
      stream_timestamp: new Date("2025-05-27T09:00:00Z"),
      duration: "1h50m",
      summary_md: `### 📝 Summary  
This session centered around a **debate on the proposal to restructure the Kenya Revenue Authority (KRA)**. Legislators expressed support and skepticism in equal measure, reflecting on past inefficiencies, accountability, and potential reforms in revenue collection.`,
      timestamp_md: "- 00:04 Introductions\n- 00:22 Revenue Authority Bill",
      is_published: true,
    },
    {
      video_id: "vFtI3O4Fa7Q",
      title: "Joint Session | Tuesday 20th May 2025 | Afternoon Session",
      view_count: "7900",
      stream_timestamp: new Date("2025-05-20T14:00:00Z"),
      duration: "2h03m",
      summary_md: `### 📝 Summary  
This joint sitting of the National Assembly and Senate addressed **issues surrounding national public safety**. Speakers discussed legislation aimed at crime prevention, emergency response infrastructure, and citizen protections.`,
      timestamp_md: "- 00:01 Joint address\n- 00:19 Public safety bill",
      is_published: true,
    },
    {
      video_id: "7yBsJfH9UXY",
      title: "Senate | Thursday 15th May 2025 | Afternoon Session",
      view_count: "3700",
      stream_timestamp: new Date("2025-05-15T14:00:00Z"),
      duration: "1h10m",
      summary_md: `### 📝 Summary  
The Senate conducted an in-depth **discussion on national climate change policies and response strategies**. Key topics included carbon emissions, environmental degradation, and the transition to renewable energy sources.`,
      timestamp_md: "- 00:03 Opening\n- 00:14 Climate strategy",
      is_published: true,
    },
    {
      video_id: "Kz3qXZV2F8s",
      title: "National Assembly | Tuesday 13th May 2025 | Morning Session",
      view_count: "4600",
      stream_timestamp: new Date("2025-05-13T09:00:00Z"),
      duration: "2h00m",
      summary_md: `### 📝 Summary  
In this session, the Assembly heard **opening statements on proposed constitutional amendments**. Members debated the scope of the changes, focusing on representation, electoral reform, and balance of power between arms of government.`,
      timestamp_md: "- 00:02 Intro\n- 00:12 Constitution amendment bill",
      is_published: true,
    },
    {
      video_id: "bRz5XCtK7jE",
      title: "Senate | Thursday 8th May 2025 | Afternoon Session",
      view_count: "2900",
      stream_timestamp: new Date("2025-05-08T14:00:00Z"),
      duration: "45m30s",
      summary_md: `### 📝 Summary  
This brief sitting of the Senate focused on **health sector reforms**, including staffing levels in public hospitals and access to affordable medication. Several motions were passed to initiate policy reviews at the county level.`,
      timestamp_md: "- 00:01 Opening\n- 00:10 Health policy motion",
      is_published: true,
    },
  ];

  await prisma.streams.createMany({
    data: streams,
    skipDuplicates: true,
  });

  console.log(`✅ Seeded ${streams.length} Parliament videos`);
}

main()
  .catch((e) => {
    console.error("❌ Error:", e);
    process.exit(1);
  })
  .finally(() => prisma.$disconnect());
