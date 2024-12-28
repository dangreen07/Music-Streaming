import type { LoaderFunctionArgs } from "@remix-run/node";
import { Await, defer, MetaFunction, redirect, useLoaderData } from "@remix-run/react";
import { Suspense, useState } from "react";
import { getSongsList } from "~/functions/songs.server";
import { hasValidSession } from "~/functions/auth.server";

import SongPlayer from "./SongPlayer";
import MainPageContent from "./MainPageContent";
import MainPageContentSkeleton from "./MainPageContentSkeleton";
import SongPlayerSkeleton from "./SongPlayerSkeleton";
import NavigationBar from "~/components/NavigationBar";

export const meta: MetaFunction = () => {
  return [
    { title: "Music Streaming App" },
    { name: "description", content: "An app for streaming music" },
  ];
};

async function getSongInfo(server_url: string, song_id: string) {
  const response = await fetch(server_url + "/song_info/" + encodeURI(song_id));
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
  const cloudFrontUrl = process.env.CLOUDFRONT_URL;
  if (cloudFrontUrl === undefined) {
    throw new Error("CLOUDFRONT_URL environment variable not set");
  }
  const songInfo = getSongInfo(process.env.SERVER_URL_FROM_SERVER??"", "5ba801e5-ab4d-48f8-9947-66ee7b63e861");
  const songsList = getSongsList(process.env.SERVER_URL_FROM_SERVER??"");

  return defer({
    server_url,
    songInfo,
    songsList: songsList,
    cloudFrontUrl
  });
};

export default function Index() {
  const { server_url, songInfo, songsList, cloudFrontUrl } = useLoaderData<typeof loader>();
  const [ currentSongID, setCurrentSongID ] = useState("5ba801e5-ab4d-48f8-9947-66ee7b63e861");

  return (
    <div className="min-h-screen bg-neutral-800 flex flex-grow flex-col w-full">
      <NavigationBar server_url={server_url} />
      <Suspense fallback={<MainPageContentSkeleton />}>
        <Await resolve={songsList}>
          {songsList => <MainPageContent cloudFrontUrl={cloudFrontUrl} songsList={songsList} setCurrentSongID={setCurrentSongID} />}
        </Await>
      </Suspense>
      <div id="padding" className="h-28" />
      <Suspense fallback={<SongPlayerSkeleton />}>
        <Await resolve={songInfo}>
          {/* Here currentSong is the id of the song */}
          {songInfo => <SongPlayer 
            currentSong={currentSongID}
            server_url={server_url}
            initial_song_duration={songInfo.song_duration}
          />}
        </Await>
      </Suspense>
    </div>
  );
}