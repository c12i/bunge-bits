import { type ClassValue, clsx } from "clsx";
import { differenceInDays, format, formatDistanceToNowStrict } from "date-fns";
import { twMerge } from "tailwind-merge";

export const titleCase = (str: string) => {
  if (str === null || str === "") return str;

  return str.replace(/\w\S*/g, (txt) => {
    return txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase();
  });
};

export const cn = (...inputs: ClassValue[]) => {
  return twMerge(clsx(inputs));
};

export const formatDate = (dateInput: Date | string | number) => {
  const date = typeof dateInput === "string" ? new Date(dateInput) : dateInput;

  const daysAgo = differenceInDays(new Date(), date);

  if (daysAgo < 7) {
    return formatDistanceToNowStrict(date, { addSuffix: true });
  } else {
    return format(date, "d MMMM yyyy");
  }
};

export const formatDuration = (duration: string) => {
  const parts = duration.split(":").map(Number);

  let hours = 0;
  let minutes = 0;

  if (parts.length === 3) {
    [hours, minutes] = parts;
  } else if (parts.length === 2) {
    [minutes] = parts;
  } else {
    return duration; // fallback for unexpected format
  }

  return `${hours ? `${hours}h ` : ""}${minutes}m`;
};

type FeedItem = {
  title: string;
  slug: string;
  date?: Date;
};

type FeedMeta = {
  title: string;
  description: string;
  baseUrl: string;
  items: FeedItem[];
};

export const toRssFeed = ({ title, description, baseUrl, items }: FeedMeta) => {
  const postItems = items
    .map(({ title, slug, date }) => {
      const link = `${baseUrl}${slug}`;
      const pubDate = date ? `<pubDate>${new Date(date).toUTCString()}</pubDate>` : "";
      return `
      <item>
        <title>${escapeXml(title)}</title>
        <link>${link}</link>
        ${pubDate}
        <guid>${link}</guid>
      </item>
    `;
    })
    .join("");

  return `<?xml version="1.0" encoding="UTF-8" ?>
  <rss version="2.0">
    <channel>
      <title>${escapeXml(title)}</title>
      <description>${escapeXml(description)}</description>
      <link>${baseUrl}/summaries/rss.xml</link>
      ${postItems}
    </channel>
  </rss>`;
};

function escapeXml(str: string): string {
  return str.replace(
    /[<>&'"]/g,
    (c) =>
      ({ "<": "&lt;", ">": "&gt;", "&": "&amp;", "'": "&apos;", '"': "&quot;" })[c] || c
  );
}
