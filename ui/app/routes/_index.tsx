import { LoaderFunctionArgs, redirect } from "@remix-run/node";

export async function loader({ request: _ }: LoaderFunctionArgs) {
  return redirect("/summaries");
}

export default function Index() {
  return null;
}
