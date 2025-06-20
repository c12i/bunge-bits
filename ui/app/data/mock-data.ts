export interface StreamMoment {
  timestamp: string;
  description: string;
}

export interface Stream {
  videoId: string;
  title: string;
  chamber: "National Assembly" | "Senate";
  date: string;
  duration: number; // in seconds
  viewCount: number;
  isPublished: boolean;
  summary: string;
  detailedSummary: string;
  keyTopics: string[];
  keyPoints: string[];
  moments: StreamMoment[];
}

export const mockStreams: Stream[] = [
  {
    videoId: "dQw4w9WgXcQ",
    title:
      "Parliamentary Session on Budget Allocation for Healthcare Infrastructure",
    chamber: "National Assembly",
    date: "2024-03-15",
    duration: 7320, // 2 hours 2 minutes
    viewCount: 12450,
    summary:
      "Members of Parliament engaged in comprehensive discussions regarding the allocation of budget resources for healthcare infrastructure improvements across Kenya. The session covered proposed increases in funding for rural health centers, medical equipment procurement, and healthcare worker training programs.",
    detailedSummary:
      "The parliamentary session began with presentations from the Ministry of Health outlining current healthcare infrastructure challenges. Members from various constituencies raised concerns about inadequate medical facilities in their regions. The debate centered on proposals to increase healthcare budget allocation by 15% for the next fiscal year. Key discussions included the need for modern medical equipment in county hospitals, expansion of community health programs, and initiatives to address the shortage of healthcare workers in rural areas. Several amendments were proposed to ensure equitable distribution of resources across all counties.",
    keyTopics: [
      "Healthcare Budget",
      "Medical Infrastructure",
      "Rural Health Centers",
      "Healthcare Workers",
    ],
    keyPoints: [
      "Proposed 15% increase in healthcare budget allocation for next fiscal year",
      "Focus on improving medical equipment in county hospitals across all regions",
      "Expansion of community health programs to reach underserved populations",
      "Initiatives to address critical shortage of healthcare workers in rural areas",
      "Amendments proposed for equitable county resource distribution among counties",
    ],
    isPublished: true,
    moments: [
      {
        timestamp: "00:15:30",
        description:
          "Minister of Health presents infrastructure assessment report",
      },
      {
        timestamp: "00:42:18",
        description:
          "MP from Turkana raises concerns about medical equipment shortages",
      },
      {
        timestamp: "01:08:45",
        description: "Debate begins on 15% budget increase proposal",
      },
      {
        timestamp: "01:35:22",
        description: "Discussion on healthcare worker shortage in rural areas",
      },
      {
        timestamp: "01:52:10",
        description:
          "Amendment proposed for equitable county resource distribution",
      },
    ],
  },
  {
    videoId: "ScMzIvxBSi4",
    title: "Climate Change and Environmental Protection Act Review",
    chamber: "Senate",
    date: "2024-06-12",
    duration: 6300, // 1h 45m
    summary:
      "Senators deliberated on amendments to the Climate Change and Environmental Protection Act, focusing on carbon emission reduction targets and renewable energy incentives. The session featured presentations from environmental experts and stakeholders from various sectors.",
    detailedSummary:
      "The Senate committee on Environment and Natural Resources led discussions on proposed amendments to strengthen Kenya's climate change response. Senators examined new provisions for carbon credit trading, forest conservation measures, and penalties for environmental violations. Key presentations were made by representatives from the Kenya Association of Manufacturers and environmental NGOs. The debate highlighted the balance between economic development and environmental protection, with specific focus on industrial emission standards and renewable energy adoption incentives.",
    keyTopics: [
      "Climate Change",
      "Environmental Protection",
      "Renewable Energy",
      "Carbon Trading",
    ],
    keyPoints: [
      "Review of carbon emission reduction targets for industrial sectors",
      "Introduction of carbon credit trading mechanisms",
      "Enhanced penalties for environmental law violations",
      "Incentives for renewable energy adoption by businesses",
      "Forest conservation and reforestation requirements",
    ],
    viewCount: 8967,
    isPublished: true,
    moments: [],
  },
  {
    videoId: "9bZkp7q19f0",
    title: "Digital Economy and ICT Infrastructure Development",
    chamber: "National Assembly",
    date: "2024-06-10",
    duration: 7200, // 2h
    summary:
      "Parliament discussed the progress of Kenya's digital transformation agenda, including broadband connectivity expansion, cybersecurity measures, and digital skills training programs. Members addressed challenges in rural internet access and e-government service delivery.",
    detailedSummary:
      "The session provided comprehensive updates on the national broadband strategy and digital literacy initiatives. MPs received detailed reports on fiber optic cable installations across rural areas and the challenges faced in last-mile connectivity. Cybersecurity emerged as a major concern, with discussions on protecting government digital infrastructure and citizen data. The debate also covered progress in digitizing government services and the impact on service delivery efficiency. Several members raised constituency-specific connectivity issues and requested intervention.",
    keyTopics: [
      "Digital Economy",
      "ICT Infrastructure",
      "Cybersecurity",
      "E-Government",
    ],
    keyPoints: [
      "Progress report on national broadband expansion to rural areas",
      "Cybersecurity measures for government digital infrastructure",
      "Digital skills training program updates and enrollment statistics",
      "Challenges in last-mile internet connectivity solutions",
      "E-government service digitization and citizen accessibility",
    ],
    viewCount: 15782,
    isPublished: true,
    moments: [],
  },
  {
    videoId: "JGwWNGJdvx8",
    title: "Youth Employment and Skills Development Initiative",
    chamber: "Senate",
    date: "2024-06-08",
    duration: 5400, // 1h 30m
    summary:
      "Senators examined the implementation of youth employment programs and skills development initiatives. The session focused on addressing youth unemployment challenges and creating pathways for economic participation through vocational training and entrepreneurship support.",
    detailedSummary:
      "The discussion centered on evaluating the effectiveness of existing youth employment programs and identifying gaps in implementation. Senators reviewed statistics showing current youth unemployment rates and the impact of government interventions. The debate featured analysis of vocational training programs, apprenticeship opportunities, and access to credit for young entrepreneurs. Members emphasized the need for stronger partnerships with private sector employers and better alignment of training programs with market demands.",
    keyTopics: [
      "Youth Employment",
      "Skills Development",
      "Entrepreneurship",
      "Vocational Training",
    ],
    keyPoints: [
      "Review of current youth unemployment statistics and trends",
      "Assessment of vocational training program effectiveness",
      "Discussion on access to credit and financing for young entrepreneurs",
      "Need for stronger public-private partnerships in youth employment",
      "Alignment of skills training with current market demands",
    ],
    viewCount: 9234,
    isPublished: true,
    moments: [],
  },
  {
    videoId: "L_jWHffIx5E",
    title: "Agricultural Modernization and Food Security Strategy",
    chamber: "National Assembly",
    date: "2024-06-05",
    duration: 9000, // 2h 30m
    summary:
      "MPs deliberated on Kenya's agricultural modernization strategy, addressing food security challenges, irrigation infrastructure, and support for smallholder farmers. The session included discussions on climate-smart agriculture and market access improvements.",
    detailedSummary:
      "The parliamentary session provided a comprehensive review of Kenya's agricultural sector transformation agenda. Discussions covered the rollout of irrigation projects, subsidized fertilizer programs, and technology adoption by smallholder farmers. MPs addressed challenges related to post-harvest losses, market access, and price volatility affecting farmers. Climate-smart agriculture practices and drought-resistant crop varieties were highlighted as key strategies for building resilience. The debate also examined export potential and value addition opportunities in the agricultural sector.",
    keyTopics: [
      "Agriculture",
      "Food Security",
      "Irrigation",
      "Climate-Smart Farming",
    ],
    keyPoints: [
      "Progress on national irrigation infrastructure development projects",
      "Subsidized fertilizer program distribution and impact assessment",
      "Technology adoption initiatives for smallholder farmers",
      "Strategies to reduce post-harvest losses and improve storage",
      "Climate-smart agriculture practices and drought-resistant crops",
    ],
    viewCount: 11456,
    isPublished: true,
    moments: [],
  },
  {
    videoId: "kfVsfOSbJY0",
    title: "Urban Planning and Housing Development Policy",
    chamber: "Senate",
    date: "2024-06-03",
    duration: 6900, // 1h 55m
    summary:
      "The Senate discussed urban planning policies and affordable housing development programs. Key topics included slum upgrading initiatives, building standards enforcement, and sustainable urban growth management.",
    detailedSummary:
      "Senators engaged in detailed discussions about Kenya's urbanization challenges and housing deficit. The session examined the progress of affordable housing projects under the Big Four Agenda and challenges in implementation. Discussions covered slum upgrading programs, their impact on residents, and relocation strategies. Building standards and enforcement mechanisms were scrutinized, with emphasis on ensuring safety and quality in construction. The debate also addressed sustainable urban planning practices and infrastructure development to support growing urban populations.",
    keyTopics: [
      "Urban Planning",
      "Affordable Housing",
      "Slum Upgrading",
      "Building Standards",
    ],
    keyPoints: [
      "Progress review of affordable housing projects and delivery timelines",
      "Slum upgrading initiatives and community relocation strategies",
      "Building standards enforcement and construction quality assurance",
      "Sustainable urban planning for growing city populations",
      "Infrastructure development to support urban expansion",
    ],
    viewCount: 7823,
    isPublished: true,
    moments: [],
  },
];
