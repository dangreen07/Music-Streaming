import { LoaderFunctionArgs, redirect } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { useState } from "react";
import NavigationBar from "~/components/NavigationBar";
import { getUser } from "~/functions/auth.server";

export async function loader({ request }: LoaderFunctionArgs) {
    const user = await getUser(request);
    if (user === null) {
        return redirect("/login");
    }
    if(user.permissions !== "admin") {
        return redirect("/login");
    }
    const server_url = process.env.SERVER_URL;
    if(server_url === undefined) {
        throw new Error("SERVER_URL environment variable not set");
    }
    return server_url;
}

export default function NewSong() {
    const server_url = useLoaderData<typeof loader>();
    const [songTitle, setSongTitle] = useState("");
    const [songArtist, setSongArtist] = useState("");
    const [songAlbum, setSongAlbum] = useState("");
    const [songFiles, setSongFiles] = useState<FileList | null>(null);
    const [songImage, setSongImage] = useState<FileList | null>(null);
    const [submitDisabled, setSubmitDisabled] = useState(false);

    async function UploadFile() {
        if (songTitle === "" || songArtist === "" || songAlbum === "" || songFiles === null || songImage === null) {
            return;
        }
        setSubmitDisabled(true);
        const formData = new FormData();
        formData.append("title", songTitle);
        formData.append("artist", songArtist);
        formData.append("album", songAlbum);
        formData.append("file", songFiles[0]);
        formData.append("image", songImage[0]);
        const response = await fetch(server_url + "/song", {
            method: "POST",
            body: formData
        });
        const body = await response.text();
        if (response.status !== 200) {
            setSubmitDisabled(false);
            alert(body);
            return;
        }
        window.location.href = "/admin";
    }

    return (
        <div className="min-h-screen bg-neutral-800 w-full flex flex-col">
            <title>New Song</title>
            <NavigationBar server_url={server_url} />
            <div className="flex flex-col gap-4 items-center justify-center w-full flex-grow">
                <h1 className="text-white text-4xl mx-auto font-bold">New Song</h1>
                <div id="new-song-form" className="flex flex-col gap-4 items-center w-full max-w-2xl mx-auto">
                    <input disabled={submitDisabled} value={songTitle} onChange={(e) => setSongTitle(e.target.value)} type="text" className="input input-bordered w-full" placeholder="Song Title" />
                    <input disabled={submitDisabled} value={songArtist} onChange={(e) => setSongArtist(e.target.value)} type="text" className="input input-bordered w-full" placeholder="Song Artist" />
                    <input disabled={submitDisabled} value={songAlbum} onChange={(e) => setSongAlbum(e.target.value)} type="text" className="input input-bordered w-full" placeholder="Song Album" />
                    <div className="flex flex-col gap-0.5 w-full">
                        <span className="text-lg font-semibold">Album Cover:</span>
                        <input disabled={submitDisabled} onChange={(e) => setSongImage(e.target.files)} type="file" className="p-4 input input-bordered w-full h-full" placeholder="Song Image" />
                    </div>
                    <div className="flex flex-col gap-0.5 w-full">
                        <span className="text-lg font-semibold">Song File:</span>
                        <input accept="audio/wav, audio/mpeg" disabled={submitDisabled} onChange={(e) => setSongFiles(e.target.files)} type="file" className="p-4 input input-bordered w-full h-full" placeholder="Song File" />
                    </div>
                    <button disabled={songTitle === "" || songArtist === "" || songAlbum === "" || songFiles === null || submitDisabled} onClick={() => UploadFile()} className="btn btn-secondary btn-lg btn-wide">Add Song</button>
                </div>
            </div>
        </div>
    );
}