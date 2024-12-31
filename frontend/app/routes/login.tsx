import { useState } from "react";
import Cookies from 'js-cookie';
import { redirect, useLoaderData } from "@remix-run/react";
import { LoaderFunctionArgs } from "@remix-run/node";
import { hasValidSession } from "../functions/auth.server";

export async function loader({ request }: LoaderFunctionArgs) {
    const isAuthenticated = await hasValidSession(request);
    if (isAuthenticated) {
        // Redirect to the home page if the user is already authenticated
        return redirect("/");
    }
    return {
        server_url: process.env.SERVER_URL,
    };
}

export default function Login() {
    // Gets environment variables needed and checks if the user is authenticated
    const environment = useLoaderData() as { server_url: string };

    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState("");

    return (
        <div className="min-h-screen flex flex-grow flex-col w-full justify-center items-center">
            <title>Music Streaming - Login</title>
            <div id="center" className="flex flex-col items-center w-full max-w-xl bg-neutral-200 py-16 px-5 rounded-3xl gap-3">
                <span className="text-4xl font-bold text-black">Login</span>
                <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} placeholder="Username..." className="input input-bordered max-w-96 w-3/4" />
                <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Password..." className="input input-bordered max-w-96 w-3/4" />
                {error != "" && <p className="text-red-500 text-md">{error}</p>}
                <button className="btn btn-secondary btn-lg btn-wide" onClick={() => {
                    fetch(environment.server_url + "/login", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json",
                        },
                        body: JSON.stringify({
                            username: username,
                            password: password,
                        }),
                    }).then(response => response.json()).then((data: { session_id: string, error: string }) => {
                        if (password.length == 0 || username.length == 0) {
                            setError("Username and password cannot be empty!");
                            return;
                        }
                        if (data.error == "")
                        {
                            // Setting the session cookie
                            Cookies.set("session_id", data.session_id, { expires: 30});
                            // Redirecting to the home page
                            window.location.href = "/";
                        }
                        else {
                            setError(data.error);
                        }
                    });
                }}>Login</button>
                <p className="text-md text-black">Don&apos;t have an account? <a href="/signup" className="underline">Sign up</a></p>
            </div>
        </div>
    )
}