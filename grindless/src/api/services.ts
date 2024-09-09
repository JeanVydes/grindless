import { AxiosResponse } from "axios";
import { api_uri, instance, Response } from "./api";

export interface SummaryResponse {
    message: string,
    model: string,
    tokens_proccesed: number,
    operation_cost_in_credits: number,
    operation_cost_in_usd: number,
    remaining_credits: number,
}

export async function summary_api(access_token: String, kind: String, text: String): Promise<AxiosResponse<Response<SummaryResponse>, any>> {
    return instance({
        method: "POST",
        url: `${api_uri}/api/services/summarize`,
        headers: {
            "Authorization": `Bearer ${access_token}`,
            "Content-Type": "application/x-www-form-urlencoded",
        },
        data: {
            kind,
            text,
        }
    })
}