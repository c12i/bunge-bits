import { redirect } from "@remix-run/node";

export async function loader() {
  return redirect("/summaries");
}

export default function Index() {
  return null;
}
