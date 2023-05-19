export function convertJsonToArray(jsonObject) {
    if(!jsonObject) {
        return [];
    }

    const jsonArray = Object.entries(jsonObject).map(([key, val]) => {
        return {
            ...val,
            key
        }
    });

    return jsonArray;
}
