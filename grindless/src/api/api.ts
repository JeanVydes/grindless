import { production, api_domain, client_domain, dev_api_domain, dev_client_domain } from '@/config.json';
import Axios, { AxiosResponse } from 'axios';

export const api_uri: string = `http${production ? "s" : ""}://${production ? api_domain : dev_api_domain}`;

export interface Response<T> {
    success: "ok" | "error",
    message: string | undefined | null,
    data: T | undefined | null,
}

export const instance = Axios.create({
    baseURL: api_uri,
    withCredentials: true,
    headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Credentials": "true",
        "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
        "Access-Control-Allow-Headers": "*",
    }
});

export async function access_google_oauth<T>(code: string): Promise<AxiosResponse<T, any>> {
    let data = new URLSearchParams();
    data.append("code", code);

    return instance({
        method: "POST",
        url: `${api_uri}/api/oauth/access/google`,
        data,
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        }
    })
}