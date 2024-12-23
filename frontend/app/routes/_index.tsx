import type { LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { hasValidSession } from "../functions/auth.server";
import Cookies from "js-cookie";
import { FaBackwardStep, FaForwardStep, FaPause } from "react-icons/fa6";

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
      <div id="player-section" className="absolute bottom-0 flex flex-col items-center justify-center w-full bg-neutral-900 p-3 gap-2">
        <div className="flex gap-3 items-center">
          <button id="prev-btn" className="text-white p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out">
            <FaBackwardStep size={32} />
          </button>
          <button id="play-pause-btn" className="bg-white text-black p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out">
            <FaPause size={32} />
          </button>
          <button id="next-btn" className="text-white p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out">
            <FaForwardStep size={32} />
          </button>
        </div>
        <div className="flex w-1/2 items-center gap-2">
          <span id="current-song-time" className="text-white text-md">0:00</span>
          <progress className="progress w-full" value={50} max="100"></progress>
          <span id="song-duration" className="text-white text-md">1:00</span>
        </div>
      </div>
    </div>
  );
}