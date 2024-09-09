import { AxiosResponse } from "axios";
import { api_uri, instance } from "./api";

export async function get_my_info<T>(access_token: String): Promise<AxiosResponse<T, any>> {
    return instance({
        method: "GET",
        url: `${api_uri}/api/accounts/@me`,
        headers: {
            "Authorization": `Bearer ${access_token}`
        }
    })
}