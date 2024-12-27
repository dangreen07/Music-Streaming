import { Song } from "~/types"

export default function MainPageContent({songsList, setCurrentSongID}: {songsList: Song[], setCurrentSongID: React.Dispatch<React.SetStateAction<string>>}) {
    return (
    <div id="content">
        <div className="flex flex-col gap-2 p-3">
            {songsList.map((song, index) => {
                return (
                    <button
                        key={index}
                        className="bg-neutral-900 px-4 p-2 rounded-3xl flex"
                        onClick={() => setCurrentSongID(song.id)}
                    >
                        <span>{song.title} by {song.artist}</span>
                    </button>
                )
            })}
        </div>
    </div>
    )
}