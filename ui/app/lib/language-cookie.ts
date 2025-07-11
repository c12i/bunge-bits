import { createCookie } from "@remix-run/node";

export const languageCookie = createCookie("lng", {
  httpOnly: true,
  secure: process.env.NODE_ENV === "production",
  sameSite: "lax",
  maxAge: 60 * 60 * 24 * 365,
});
