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
  {
    videoId: "3JZ_D3ELwOQ",
    title: "Education Funding and Curriculum Reform Debate",
    chamber: "National Assembly",
    date: "2024-05-30",
    duration: 8100,
    viewCount: 10342,
    summary:
      "Members of Parliament debated proposed changes to the national education curriculum and funding allocation models for public schools. Discussions included capitation grants, teacher training, and infrastructure upgrades.",
    detailedSummary:
      "The session began with a Ministry of Education briefing on the outcomes of recent curriculum reviews. MPs raised concerns about disparities in capitation disbursement and infrastructure quality across counties. Proposals to review the Competency-Based Curriculum (CBC) implementation timeline were introduced. Several legislators emphasized the need for modern laboratories, digital learning tools, and targeted teacher training in underserved areas. An amendment to increase funding to marginalized regions was tabled.",
    keyTopics: [
      "Education Reform",
      "CBC Curriculum",
      "School Funding",
      "Teacher Training",
    ],
    keyPoints: [
      "Review of capitation funding models for public primary and secondary schools",
      "Concerns over CBC implementation logistics in rural areas",
      "Push for increased investment in school infrastructure and technology",
      "Emphasis on teacher training and professional development",
      "Call for equitable funding distribution across all counties",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "e-ORhEE9VVg",
    title: "Public Finance Management and Auditor General Report Review",
    chamber: "Senate",
    date: "2024-05-28",
    duration: 7200,
    viewCount: 6134,
    summary:
      "Senators reviewed the latest Auditor General’s report, examining misuse of funds by county governments and compliance with Public Finance Management (PFM) regulations.",
    detailedSummary:
      "The debate focused on audit findings that highlighted procurement irregularities and unaccounted expenditures in several counties. Senators called for stricter oversight mechanisms and enforcement of PFM laws. Proposals included real-time digital tracking of county spending and routine capacity-building programs for county finance officers. Recommendations to establish independent monitoring units under the Office of the Controller of Budget were discussed.",
    keyTopics: [
      "Auditor General Report",
      "Public Finance",
      "County Governance",
      "Budget Oversight",
    ],
    keyPoints: [
      "Irregular procurement flagged in 14 counties",
      "Need for real-time financial tracking systems",
      "Proposal for enhanced budget officer training",
      "Calls for criminal investigations on misuse cases",
      "Recommendation to establish regional audit hubs",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "RgKAFK5djSk",
    title: "Security Sector Reforms and Police Welfare Motion",
    chamber: "National Assembly",
    date: "2024-05-25",
    duration: 6600,
    viewCount: 8421,
    summary:
      "MPs debated proposed reforms within the security sector, focusing on police welfare, housing, and institutional accountability.",
    detailedSummary:
      "The session addressed systemic challenges faced by police officers, including poor housing, inadequate pay, and psychological support services. Legislators examined a motion proposing the restructuring of the National Police Service Commission to enhance transparency and autonomy. Discussions also touched on community policing strategies and use-of-force guidelines. A committee was tasked with drafting a comprehensive Police Welfare Bill.",
    keyTopics: [
      "Security Reforms",
      "Police Welfare",
      "Community Policing",
      "Institutional Accountability",
    ],
    keyPoints: [
      "Proposal to restructure National Police Service Commission",
      "Debate on minimum housing standards for officers",
      "Concerns about mental health support services",
      "Introduction of guidelines on responsible use of force",
      "Establishment of Police Welfare Bill drafting committee",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "hT_nvWreIhg",
    title: "Gender Equality and Women's Economic Empowerment Bill",
    chamber: "Senate",
    date: "2024-05-20",
    duration: 6000,
    viewCount: 5480,
    summary:
      "The Senate debated the Gender Equality and Women’s Economic Empowerment Bill, which aims to improve access to finance, land rights, and representation for women.",
    detailedSummary:
      "Senators highlighted gender disparities in access to land ownership, credit facilities, and formal employment. The bill proposes affirmative action incentives for businesses that support women-led ventures, and quotas for women in public procurement. Several members emphasized the importance of implementing gender-responsive budgeting at both national and county levels.",
    keyTopics: [
      "Gender Equality",
      "Economic Empowerment",
      "Land Rights",
      "Affirmative Action",
    ],
    keyPoints: [
      "Quotas for women in public procurement",
      "Calls for gender-responsive budgeting frameworks",
      "Discussion on cultural barriers to land ownership",
      "Incentives for private sector gender inclusion",
      "Support for women-led SMEs and cooperatives",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "60ItHLz5WEA",
    title: "National Disaster Preparedness and Response Strategy",
    chamber: "National Assembly",
    date: "2024-05-15",
    duration: 7200,
    viewCount: 6920,
    summary:
      "Parliament discussed strategies for improving national disaster preparedness and response, including flood mitigation, drought resilience, and emergency services capacity.",
    detailedSummary:
      "The session reviewed the National Disaster Risk Reduction framework and highlighted the need for improved early warning systems and inter-agency coordination. MPs proposed increased funding for NDOC and county disaster response units. Topics included school-based disaster drills, community evacuation protocols, and the role of the military in relief logistics. Several members pushed for climate-proof infrastructure design in high-risk zones.",
    keyTopics: [
      "Disaster Management",
      "Floods and Drought",
      "Emergency Services",
      "Climate Resilience",
    ],
    keyPoints: [
      "Increased NDOC funding and decentralized emergency response",
      "Proposals for nationwide school safety drills",
      "Need for modernized early warning systems",
      "Use of GIS for flood risk mapping",
      "Integration of climate resilience into infrastructure projects",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "2vjPBrBU-TM",
    title: "National Cohesion and Ethnic Relations Commission Review",
    chamber: "Senate",
    date: "2024-05-10",
    duration: 5400,
    viewCount: 4130,
    summary:
      "Senators conducted a review of the NCIC’s mandate and effectiveness in addressing ethnic polarization and promoting peaceful coexistence across counties.",
    detailedSummary:
      "The Senate debated challenges faced by the National Cohesion and Integration Commission (NCIC) in implementing conflict resolution strategies and monitoring hate speech. Members proposed increased funding, grassroots peace campaigns, and inter-county reconciliation forums. Discussions also addressed political incitement during elections and how to safeguard cohesion using both legal and civic tools.",
    keyTopics: [
      "National Cohesion",
      "Ethnic Relations",
      "Hate Speech",
      "Peacebuilding",
    ],
    keyPoints: [
      "Strengthening NCIC’s investigative and enforcement powers",
      "Grassroots-based civic education campaigns",
      "Proposal for digital hate speech monitoring platforms",
      "Inter-county peacebuilding and dialogue forums",
      "Legislative support for cohesion education in schools",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "uelHwf8o7_U",
    title: "Transport Infrastructure and Road Safety Reforms",
    chamber: "National Assembly",
    date: "2024-05-07",
    duration: 7500,
    viewCount: 10923,
    summary:
      "MPs discussed road safety reforms, infrastructure project updates, and public transport regulation frameworks, with a focus on reducing road fatalities and enhancing urban mobility.",
    detailedSummary:
      "The session began with a report from NTSA highlighting alarming road crash statistics. Lawmakers debated proposals for stricter licensing requirements, digital monitoring of matatus, and funding gaps in road expansion projects. Particular focus was placed on blackspot identification, non-motorized transport lanes, and penalties for traffic law violations.",
    keyTopics: [
      "Road Safety",
      "Urban Mobility",
      "Public Transport",
      "Infrastructure Projects",
    ],
    keyPoints: [
      "NTSA road crash report sparked urgent safety reforms",
      "Digital monitoring systems proposed for matatus and PSVs",
      "Funding updates for key expressway and bypass projects",
      "Support for dedicated cycling and pedestrian lanes",
      "Calls for harsher penalties for repeat traffic offenders",
    ],
    isPublished: true,
    moments: [],
  },
  {
    videoId: "ktvTqknDobU",
    title: "Taxation Policy and Finance Bill Stakeholder Consultations",
    chamber: "Senate",
    date: "2024-05-04",
    duration: 8400,
    viewCount: 9630,
    summary:
      "Senators reviewed public submissions on the Finance Bill and its taxation proposals, including digital services tax, VAT adjustments, and relief measures for low-income earners.",
    detailedSummary:
      "The session featured feedback from the Kenya Revenue Authority, private sector associations, and civil society groups. Senators raised concerns about the digital services tax burden on SMEs and proposed tiered VAT structures to protect essential goods. Discussions also touched on excise duty reforms and tax amnesty programs. The need for clear implementation timelines and public education was emphasized.",
    keyTopics: ["Taxation", "Finance Bill", "VAT", "Digital Services"],
    keyPoints: [
      "Public feedback highlights risks of overtaxing SMEs",
      "Proposal for tiered VAT rates to protect essentials",
      "Revisions to digital services tax threshold",
      "Debate on implementation of tax amnesty policy",
      "Emphasis on transparent public engagement processes",
    ],
    isPublished: true,
    moments: [],
  },
];
