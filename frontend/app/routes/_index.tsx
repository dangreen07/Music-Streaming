import type { LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { hasValidSession } from "../functions/auth.server";
import Cookies from "js-cookie";

export const meta: MetaFunction = () => {
  return [
    { title: "Music Streaming App" },
    { name: "description", content: "An app for streaming music" },
  ];
};

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const isAuthenticated = await hasValidSession(request);
  if (!isAuthenticated) {
    return redirect("/login");
  }
  return {
    server_url: process.env.SERVER_URL,
  };
};

export default function Index() {
  const environment = useLoaderData() as { server_url: string };

  return (
    <div className="min-h-screen bg-neutral-800 flex flex-grow flex-col w-full">
      <div id="navbar" className="flex items-center justify-between w-full bg-neutral-900 p-3">
        <div id="left-side">
          <img src="/logo-removebg.png" alt="logo" className="h-16 w-16" />
        </div>
        <div id="right-side">
          <button className="btn btn-primary btn-md" onClick={async () => {
            console.log(Cookies.get("session_id"));
            await fetch(environment.server_url + "/logout", {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({
                session_id: Cookies.get("session_id"),
              })
            });
            Cookies.remove("session_id");
            window.location.href = "/";
          }}>Logout</button>
        </div>
      </div>
    </div>
  );
}