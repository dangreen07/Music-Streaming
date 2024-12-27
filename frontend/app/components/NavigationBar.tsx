import Cookies from "js-cookie";

export default function NavigationBar({ server_url }: { server_url: string }) {
    return (
    <div id="navbar" className="flex items-center justify-between w-full bg-neutral-900 p-3">
        <div id="left-side">
          <button className="btn btn-ghost btn-lg btn-square" onClick={() => window.location.href = "/"}>
            <img src="/logo-removebg.png" alt="logo" className="h-16 w-16" />
          </button>
        </div>
        <div id="right-side">
          <button className="btn btn-primary btn-md" onClick={async () => {
            await fetch(server_url + "/logout", {
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
    )
}