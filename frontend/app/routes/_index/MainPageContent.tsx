import { FaMusic } from "react-icons/fa6"
import { Song } from "~/types"

export default function MainPageContent({songsList, setCurrentSongID, cloudFrontUrl}: {songsList: Song[], setCurrentSongID: React.Dispatch<React.SetStateAction<string>>, cloudFrontUrl: string}) {
    return (
    <div id="content">
        <div className="flex flex-col gap-2 p-3">
            {songsList.map((song, index) => {
                return (
                    <button
                        key={index}
                        className="bg-neutral-900 px-4 p-2 rounded-md flex gap-2"
                        onClick={() => setCurrentSongID(song.id)}
                    >
                        <object data={`${cloudFrontUrl}/${song.id}/${song.id}.png`} type="image/png" className="w-16 h-16 rounded-md">
                            <div className="w-16 h-16 flex justify-center items-center">
                                <FaMusic size={32} />
                            </div>
                        </object>
                        <div className="flex flex-col h-16 justify-center items-start">
                            <span className="text-md text-white font-semibold">{song.title}</span>
                            <span className="text-gray-400 text-sm">{song.artist}</span>
                        </div>
                    </button>
                )
            })}
        </div>
    </div>
    )
}