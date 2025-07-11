import type { ActionFunctionArgs } from "@remix-run/node";
import { createCookie, redirect } from "@remix-run/node";

const languageCookie = createCookie("lng", {
  httpOnly: true,
  secure: process.env.NODE_ENV === "production",
  sameSite: "lax",
  maxAge: 60 * 60 * 24 * 365,
});

export async function action({ request }: ActionFunctionArgs) {
  const formData = await request.formData();
  const language = formData.get("language") as string;
  const redirectTo = formData.get("redirectTo") as string;

  if (!language || !["en", "sw"].includes(language)) {
    return redirect("/");
  }

  // Set the language cookie
  const cookieHeader = await languageCookie.serialize(language);

  return redirect(redirectTo || request.headers.get("Referer") || "/", {
    headers: {
      "Set-Cookie": cookieHeader,
    },
  });
}
