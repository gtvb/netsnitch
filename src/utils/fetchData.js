import { listen }  from "@tauri-apps/api/event";
import { convertJsonToArray } from "../utils/json";

export async function fetchUsageData(event) {
    let data;
    const unlisten = await listen(event, (data) => {
        data = convertJsonToArray(JSON.parse(data.payload));
    })

    return {
        data,
        unlisten
    }
}
