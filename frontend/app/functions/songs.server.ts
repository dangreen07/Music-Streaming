import { Song } from "~/types";

export async function getSongsList(server_url: string) {
    const response = await fetch(server_url + "/songs_list",
      {
          method: "GET"
      }
    );
    const json = await response.json() as Song[];
    return json;
  }