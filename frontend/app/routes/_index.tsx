import type { LoaderFunctionArgs } from "@remix-run/node";
import { Await, defer, MetaFunction, redirect, useLoaderData } from "@remix-run/react";
import { hasValidSession } from "../functions/auth.server";
import NavigationBar from "~/components/NavigationBar";
import SongPlayer from "~/components/SongPlayer";
import MainPageContent from "~/components/MainPageContent";
import { Suspense } from "react";
import MainPageContentSkeleton from "~/components/MainPageContentSkeleton";
import SongPlayerSkeleton from "~/components/SongPlayerSkeleton";

export const meta: MetaFunction = () => {
  return [
    { title: "Music Streaming App" },
    { name: "description", content: "An app for streaming music" },
  ];
};

async function getSongsList(server_url: string) {
  const response = await fetch(server_url + "/songs_list",
    {
        method: "GET"
    }
  );
  const json = await response.json() as string[];
  return json;
}

async function getSongInfo(server_url: string, song_name: string) {
  const response = await fetch(server_url + "/song_info/" + encodeURI(song_name));
  const songInfo = await response.json() as { song_duration: number };
  return songInfo;
}

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const isAuthenticated = await hasValidSession(request);
  if (!isAuthenticated) {
    return redirect("/login");
  }
  const server_url = process.env.SERVER_URL;
  if (server_url === undefined) {
    throw new Error("SERVER_URL environment variable not set");
  }
  const songInfo = getSongInfo(server_url, "Crab Rave.wav");
  const songsList = getSongsList(server_url);

  return defer({
    server_url,
    songInfo,
    songsList: songsList
  });
};

export default function Index() {
  const { server_url, songInfo, songsList } = useLoaderData<typeof loader>();

  return (
    <div className="min-h-screen bg-neutral-800 flex flex-grow flex-col w-full">
      <NavigationBar server_url={server_url} />
      <Suspense fallback={<MainPageContentSkeleton />}>
        <Await resolve={songsList}>
          {songsList => <MainPageContent songsList={songsList} />}
        </Await>
      </Suspense>
      <Suspense fallback={<SongPlayerSkeleton />}>
        <Await resolve={songInfo}>
          {songInfo => <SongPlayer server_url={server_url} initial_song_duration={songInfo.song_duration} />}
        </Await>
      </Suspense>
    </div>
  );
}