import type { LoaderFunctionArgs, MetaFunction } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { hasValidSession } from "../functions/auth.server";
import Cookies from "js-cookie";
import { FaBackwardStep, FaForwardStep, FaPause, FaPlay } from "react-icons/fa6";
import { useEffect, useState } from "react";

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

function formatTime(seconds: number): string {
  const roundedSeconds = Math.round(seconds);
  const minutes = Math.floor(roundedSeconds / 60);
  const remainingSeconds = roundedSeconds % 60;
  const formattedSeconds = remainingSeconds.toString().padStart(2, '0');
  return `${minutes}:${formattedSeconds}`;
}

export default function Index() {
  const environment = useLoaderData() as { server_url: string };
  const [audioContext, setAudioContext] = useState<AudioContext | null>(null);
  const [currentTime, setCurrentTime] = useState(0);
  const [songDuration, setSongDuration] = useState(1);
  const [playing, setPlaying] = useState(false);
  const [currentSample, setCurrentSample] = useState(0);
  const [volumeNum, setVolumeNum] = useState(50);

  async function GetSongInfo() {
    const response = await fetch(environment.server_url + "/sample_info");
    if (response.status !== 200) {
      return null;
    }
    const songInfo = await response.json();
    setSongDuration(songInfo.song_duration);
  }

  async function GetAudio(sample_number: number = 0) {
    const response = await fetch(environment.server_url + "/sample/" + sample_number);
    if (response.status !== 200) {
      return null;
    }
    const audioBuffer = await response.arrayBuffer();
    return audioBuffer;
  }

  async function decodeAudioChunk(arrayBuffer: ArrayBuffer) {
    if (audioContext === null)
      return;
    return audioContext.decodeAudioData(arrayBuffer);
  }

  function playAudioChunk(audioBuffer: AudioBuffer, startTime: number) {
    if (audioContext === null)
      return;
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    const gainNode = audioContext?.createGain();
    if (gainNode)
    {
      gainNode.gain.value = 0.5;
      gainNode.connect(audioContext.destination);
      source.connect(gainNode);
    }
    source.start(startTime);
  }

  async function playAudio() {
    await nextSample();
  }

  function togglePlay() {
    if(playing) {
      audioContext?.suspend();
      setPlaying(false);
    }
    else if (audioContext?.currentTime == 0) {
      playAudio();
      setPlaying(true);
    }
    else {
      audioContext?.resume();
      setPlaying(true);
    }
  }

  async function nextSample(first_run: boolean = false) {
    setCurrentSample(currentSample + 1);
    const sample_current = first_run ? 0 : currentSample;
    console.log("Next sample");
    const audioBuffer = await GetAudio(sample_current);
    if (audioBuffer === null) {
      return;
    }
    const decodedAudioBuffer = await decodeAudioChunk(audioBuffer);
    if (decodedAudioBuffer) {
      playAudioChunk(decodedAudioBuffer, sample_current * 10);
    }
  }

  useEffect(() => {
    const temp = new AudioContext();
    setAudioContext(temp);
    GetSongInfo();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const interval = setInterval(async () => {
      const currentTime = audioContext?.currentTime ?? 0;
      // Check whether next sample needs to be fetched
      // This fetches the next sample 8 seconds before it is needed
      if ((currentTime >= currentSample * 10 - 5) && playing) {
        await nextSample();
      }
      if (currentTime >= songDuration && playing) {
        audioContext?.close();
        setCurrentTime(0);
        setCurrentSample(0);
        setPlaying(false);
      }
      setCurrentTime(currentTime);
    }, 125);
    return () => clearInterval(interval);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [audioContext?.currentTime, currentSample]);

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
      <div id="player-section" className="absolute bottom-0 flex items-center justify-between w-full bg-neutral-900 px-8 py-3 gap-2">
        <div></div>
        <div className="flex flex-col items-center justify-center gap-2 w-1/2">
          <div className="flex gap-3 items-center">
            <button id="prev-btn" className="text-white p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out">
              <FaBackwardStep size={32} />
            </button>
            <button
              id="play-pause-btn"
              className="bg-white text-black p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out"
              onClick={() => {
                togglePlay();
              }}
              >
                {playing ? 
                <FaPause size={32} /> :
                <div className="pl-0.5 w-8 h-8"><FaPlay size={32} /></div>
                }
            </button>
            <button id="next-btn" className="text-white p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out">
              <FaForwardStep size={32} />
            </button>
          </div>
          <div className="flex w-full items-center gap-2">
            <span id="current-song-time" className="text-white text-md w-10">{formatTime(currentTime)}</span>
            <progress className="progress w-full" value={currentTime} max={songDuration}></progress>
            <span id="song-duration" className="text-white text-md">{formatTime(songDuration)}</span>
          </div>
        </div>
        <div id="right-section" className="flex">
          <input type="range" min={0} max="100" value={volumeNum} onChange={(current) => setVolumeNum(Number(current.target.value))} className="range" />
        </div>
      </div>
    </div>
  );
}