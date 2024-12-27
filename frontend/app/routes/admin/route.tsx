import { LoaderFunctionArgs } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { useState } from "react";
import NavigationBar from "~/components/NavigationBar";
import { getUser } from "~/functions/auth.server"
import { getSongsList } from "~/functions/songs.server";

export async function loader({ request }: LoaderFunctionArgs) {
    const user = await getUser(request);
    if (user === null) {
        return redirect("/login");
    }
    if(user.permissions !== "admin") {
        return redirect("/login");
    }
    const server_url = process.env.SERVER_URL_FROM_SERVER;
    if(server_url === undefined) {
        throw new Error("SERVER_URL_FROM_SERVER environment variable not set");
    }
    const songsList = await getSongsList(server_url);
    return {
        songsList,
        server_url: process.env.SERVER_URL??""
    }
}

export default function Admin() {
    const { songsList, server_url } = useLoaderData<typeof loader>();
    const [disabled, setDisabled] = useState(false);

    return (
        <div className="min-h-screen bg-neutral-800 flex flex-grow flex-col w-full gap-4">
            <title>Admin Console</title>
            <NavigationBar server_url={server_url} />
            <h1 className="text-white text-4xl mx-auto font-semibold">Admin Console</h1>
            <div className="flex flex-col items-center gap-2 w-full max-w-2xl mx-auto">
                <div className="flex justify-between w-full">
                    <h2 className="text-white text-2xl">Songs List</h2>
                    <button onClick={() => {
                        window.location.href = "/new-song";
                    }} className="btn btn-primary btn-sm">Add Song</button>
                </div>
                <div id="songs-list" className="w-full flex flex-col gap-2">
                    {songsList.map((current, index) => {
                        return (
                            <div key={index} className="bg-neutral-900 flex p-2 rounded-2xl w-full justify-between items-center">
                                <span>{current.title} by {current.artist}</span>
                                <button disabled={disabled} onClick={async () => {
                                    setDisabled(true);
                                    const response = await fetch(`${server_url}/song/${current.id}`,
                                        {
                                            method: "DELETE"
                                        }
                                    );
                                    if (response.status === 200) {
                                        window.location.reload();
                                    }
                                    else {
                                        alert("Error deleting song");
                                    }
                                }} className="btn btn-error btn-sm">Delete</button>
                            </div>
                        )
                    })}
                </div>
            </div>
        </div>
    )
}