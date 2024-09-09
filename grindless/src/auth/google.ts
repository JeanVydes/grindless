export const get_oauth_url_endpoint = "https://accounts.google.com/o/oauth2/v2/auth";
import {
    google_client_id,
    google_redirect_uris,
} from './config.json';

import { production } from '@/config.json';

export function GetOAuthGoogleURL(state: any) {
    const options = {
        client_id: google_client_id,
        redirect_uri: production ? google_redirect_uris[0] : google_redirect_uris[1],
        response_type: "code",
        scope: "profile email",
        //access_type: "offline",
        prompt: "consent",
        //state,
    };

    const qs = new URLSearchParams(options).toString();

    return `${get_oauth_url_endpoint}?${qs}`;
}