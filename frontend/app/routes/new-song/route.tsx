import { LoaderFunctionArgs, redirect } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { useState } from "react";
import { getUser } from "~/functions/auth.server";
import pako from 'pako';

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
    const [submitDisabled, setSubmitDisabled] = useState(false);

    async function UploadFile() {
        if (songTitle === "" || songArtist === "" || songAlbum === "" || songFiles === null) {
            return;
        }
        setSubmitDisabled(true);
        const formData = new FormData();
        formData.append("title", songTitle);
        formData.append("artist", songArtist);
        formData.append("album", songAlbum);
        formData.append("file", songFiles[0]);
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
        <div className="min-h-screen bg-neutral-800 flex flex-grow flex-col justify-center w-full p-4 gap-4">
            <title>New Song</title>
            <h1 className="text-white text-4xl mx-auto font-bold">New Song</h1>
            <div id="new-song-form" className="flex flex-col gap-4 items-center w-full max-w-2xl mx-auto">
                <input value={songTitle} onChange={(e) => setSongTitle(e.target.value)} type="text" className="input input-bordered w-full" placeholder="Song Title" />
                <input value={songArtist} onChange={(e) => setSongArtist(e.target.value)} type="text" className="input input-bordered w-full" placeholder="Song Artist" />
                <input value={songAlbum} onChange={(e) => setSongAlbum(e.target.value)} type="text" className="input input-bordered w-full" placeholder="Song Album" />
                <input onChange={(e) => setSongFiles(e.target.files)} type="file" className="p-4 input input-bordered w-full h-full" placeholder="Song File" />
                <button disabled={songTitle === "" || songArtist === "" || songAlbum === "" || songFiles === null || submitDisabled} onClick={() => UploadFile()} className="btn btn-secondary btn-lg btn-wide">Add Song</button>
            </div>
        </div>
    )
}