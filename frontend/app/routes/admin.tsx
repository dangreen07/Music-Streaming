import { LoaderFunctionArgs } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { hasValidSession } from "~/functions/auth.server";

export const loader = async ({ request }: LoaderFunctionArgs) => {
    const isAuthenticated = await hasValidSession(request);
    if (!isAuthenticated) {
        return redirect("/login");
    }
    return null;
};

export default function Admin() {
    useLoaderData();

    return (
        <div id="main">
            <title>Admin Console</title>
        </div>
    )
}