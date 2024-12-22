function parseCookieHeader(cookieHeader: string) {
    const splitCookies = cookieHeader.split(";");
    const keyValuePars = splitCookies.map(pair => pair.split("="));
    const cookies = keyValuePars.reduce((acc, v) => {
        acc[decodeURIComponent(v[0].trim())] = decodeURIComponent(v[1].trim());
        return acc;
    }, {} as Record<string, string>);
    return cookies;
}

export async function hasValidSession(request: Request) {
    const cookieHeader = request.headers.get("Cookie");
    if (cookieHeader !== null) {
      const cookies = parseCookieHeader(cookieHeader);
      const session = cookies["session_id"];
      if (session) {
        const response = await fetch(process.env.SERVER_URL_FROM_SERVER + "/validate_session", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            session_id: session,
          }),
        });
        const response_body = await response.text();
        if (response_body == "true") {
          return true;
        }
      }
  }
  return false;
}