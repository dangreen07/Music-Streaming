import { LoaderFunctionArgs } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { hasValidSession } from "../functions/auth.server";
import { useState } from "react";
import Cookies from "js-cookie";

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

export default function Signup() {
    // Gets environment variables needed and checks if the user is authenticated
    const environment = useLoaderData() as { server_url: string };
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [error, setError] = useState("");
    
    return (
        <div className="min-h-screen flex flex-grow flex-col w-full justify-center items-center">
            <title>Music Streaming - Sign Up</title>
            <div id="center" className="flex flex-col items-center w-full max-w-xl bg-neutral-200 py-16 px-5 rounded-3xl gap-3">
                <span className="text-4xl font-bold text-black">Sign up</span>
                <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} placeholder="Username..." className="input input-bordered max-w-96 w-3/4" />
                <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Password..." className="input input-bordered max-w-96 w-3/4" />
                <input type="password" value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} placeholder="Confirm Password..." className="input input-bordered max-w-96 w-3/4" />
                {error != "" && <p className="text-red-500 text-md">{error}</p>}
                <button className="btn btn-secondary btn-lg btn-wide" onClick={() => {
                    if (password != confirmPassword) {
                        setError("Passwords do not match!");
                        return;
                    }
                    else if (password.length == 0 || username.length == 0) {
                        setError("Username and password cannot be empty!");
                        return;
                    }
                    else if (password.length < 8) {
                        setError("Password must be at least 8 characters long!");
                        return;
                    }
                    else if (username.length < 3) {
                        setError("Username must be at least 3 characters long!");
                        return;
                    }
                    fetch(environment.server_url + "/signup", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json",
                        },
                        body: JSON.stringify({
                            username: username,
                            password: password,
                        }),
                    }).then(response => response.json()).then((data: { session_id: string, error: string }) => {
                        if (data.error == "")
                        {
                            // Setting the session cookie
                            Cookies.set("session_id", data.session_id);
                            // Redirecting to the home page
                            window.location.href = "/";
                        }
                        else {
                            setError(data.error);
                        }
                    });
                }}>Sign up</button>
                <p className="text-md text-black">Already have an account? <a href="/login" className="underline">Login</a></p>
            </div>
        </div>
    )
}