import { useEffect, useState } from "react";
import { FaBackwardStep, FaForwardStep, FaPause, FaPlay, FaVolumeHigh } from "react-icons/fa6";
import pako from 'pako';
import { Song } from "~/types";
import { HiOutlineQueueList } from "react-icons/hi2";

function formatTime(seconds: number): string {
    const roundedSeconds = Math.round(seconds);
    const minutes = Math.floor(roundedSeconds / 60);
    const remainingSeconds = roundedSeconds % 60;
    const formattedSeconds = remainingSeconds.toString().padStart(2, '0');
    return `${minutes}:${formattedSeconds}`;
}

export default function SongPlayer({
  server_url,
  currentSong,
  song_info,
  cloudFrontUrl
}: {
  server_url: string,
  song_info: Song,
  currentSong: string,
  cloudFrontUrl: string
}) {
    const [audioContext, setAudioContext] = useState<AudioContext | null>(null);
    const [gainNode, setGainNode] = useState<GainNode | null>(null);
    const [currentTime, setCurrentTime] = useState(0);
    const [playing, setPlaying] = useState(false);
    const [currentSample, setCurrentSample] = useState(-1);
    const [volumeNum, setVolumeNum] = useState(50);
    const [loadedSamples, setLoadedSamples] = useState(0); // 0 means no samples have been loaded
    const [songInfo, setSongInfo] = useState(song_info);
    const [songImage, setSongImage] = useState(`${cloudFrontUrl}/${currentSong}/${currentSong}.png`);

    async function GetAudio(sample_number: number = 0) {
      const response = await fetch(server_url + "/sample_compressed/" + encodeURI(currentSong) + "/" + sample_number);
      if (response.status !== 200) {
          return null;
      }
      const encodedAudioBuffer = await response.arrayBuffer();
      const inflator = new pako.Inflate();
      inflator.push(encodedAudioBuffer);
      if (inflator.err) {
        console.log(inflator.msg);
      }
      const audioBuffer = inflator.result as Uint8Array<ArrayBufferLike>;

      setLoadedSamples(prev => prev + 1);
      return audioBuffer.buffer;
    }

    async function GetSongInfo() {
      const response = await fetch(server_url + "/song_info/" + encodeURI(currentSong));
      const songInfo = await response.json() as Song;
      // Update the song image once the song info is loaded
      setSongImage(`${cloudFrontUrl}/${currentSong}/${currentSong}.png`);
      setSongInfo(songInfo);
    }

    async function decodeAudioChunk(arrayBuffer: ArrayBuffer) {
      if (audioContext === null)
          return;
      return audioContext.decodeAudioData(arrayBuffer);
    }

    function playAudioChunk(audioBuffer: AudioBuffer, startTime: number) {
      if (audioContext === null || gainNode === null)
          return;
      const source = audioContext.createBufferSource();
      source.buffer = audioBuffer;
      source.connect(gainNode);
      source.start(startTime);
    }

    async function playAudio() {
      if (audioContext === null || audioContext.state === 'closed') {
          const temp = new AudioContext();
          const tempGainNode = temp.createGain();
          temp.suspend();
          tempGainNode.gain.value = volumeNum / 100;
          tempGainNode.connect(temp.destination);
          setAudioContext(temp);
          setGainNode(tempGainNode);
      }
      if (audioContext?.state === 'suspended') {
          await audioContext.resume();
          return;
      }
    }

    function togglePlay() {
      if (playing) {
          audioContext?.suspend();
          setPlaying(false);
      }
      else {
          playAudio();
          setPlaying(true);
      }
    }

    async function nextSample(sampleNumber: number) {
      // Ensure we are still playing
      if (audioContext === null) return;
      const audioBuffer = await GetAudio(sampleNumber);
      if (audioBuffer === null) {
          return;
      }
      const decodedAudioBuffer = await decodeAudioChunk(audioBuffer);
      if (decodedAudioBuffer) {
          playAudioChunk(decodedAudioBuffer, sampleNumber * 10);
      }
    }

    async function stopAudio() {
      await audioContext?.close();
      setAudioContext(null);
      setGainNode(null);
      setCurrentTime(0);
      setCurrentSample(-1);
      setLoadedSamples(0);
      setPlaying(false);
    }

    useEffect(() => {
      stopAudio();
      GetSongInfo();
    }, [currentSong]);

    useEffect(() => {
      // Update gain value when volumeNum changes
      if (gainNode) {
          gainNode.gain.value = volumeNum / 100;
      }
    }, [volumeNum, gainNode]);

    useEffect(() => {
      const interval = setInterval(async () => {
          if (playing && audioContext) {
            const currentTime = audioContext.currentTime;
            if (currentTime >= loadedSamples * 10) {
                // The next sample is not loaded yet, so suspend the audio context
                audioContext.suspend();
            }
            else {
                audioContext.resume();
            }
            // Fetch next sample 5 seconds before it's needed
            if (currentTime >= (currentSample + 1) * 10 - 5) {
                setCurrentSample(prev => prev + 1);
                await nextSample(currentSample + 1);
            }
            if (currentTime >= songInfo.duration) {
                stopAudio();
            } else {
                setCurrentTime(currentTime);
            }
          }
      }, 125);
      return () => clearInterval(interval);
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [playing, audioContext, currentSample, loadedSamples]);

    return (
        <div id="player-section" className="fixed bottom-0 flex items-center justify-between w-full bg-neutral-900 p-3 gap-2">
        <div className="flex w-1/4 gap-3 justify-start items-center">
          <div className="w-16 h-16">
            <img src={songImage} alt="Current Song Cover" className="w-full h-full rounded-lg shadow-lg" />
          </div>
          <div className="flex flex-col justify-center">
            <span className="text-white text-md font-semibold">{songInfo.title}</span>
            <span className="text-gray-400 text-sm">{songInfo.artist}</span>
          </div>
        </div>
        <div className="flex flex-col items-center justify-center gap-2 w-1/2">
          <div className="flex gap-3 items-center">
            <button id="prev-btn" className="text-white p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out">
              <FaBackwardStep size={32} />
            </button>
            <button
              id="play-pause-btn"
              className="bg-white text-black p-2 rounded-full transition-transform active:scale-90 duration-200 ease-out"
              onClick={() => togglePlay()}
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
            <span id="current-song-time" className="text-white text-md w-12">{formatTime(currentTime)}</span>
            <progress className="progress w-full" value={currentTime} max={songInfo.duration}></progress>
            <span id="song-duration" className="text-white text-md">{formatTime(songInfo.duration)}</span>
          </div>
        </div>
        <div id="right-section" className="flex w-1/4 gap-4 items-center">
          <button className="btn btn-sm btn-ghost btn-circle"><HiOutlineQueueList size={24} /></button>
          <button className="btn btn-sm btn-ghost btn-circle"><FaVolumeHigh size={24} /></button>
          <input type="range" min={0} max="100" value={volumeNum} onChange={(current) => setVolumeNum(Number(current.target.value))} className="range range-xs" />
        </div>
      </div>
    )
}