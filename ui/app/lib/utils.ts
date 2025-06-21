import { type ClassValue, clsx } from "clsx";
import { format } from "date-fns";
import { twMerge } from "tailwind-merge";

export const cn = (...inputs: ClassValue[]) => {
  return twMerge(clsx(inputs));
};

export const formatDate = (dateString: string) => {
  return format(new Date(dateString), "d MMMM yyyy");
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
