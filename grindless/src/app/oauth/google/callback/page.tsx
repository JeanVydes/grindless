"use client"
import { access_google_oauth, Response } from "@/api/api";
import { DarkModeLoading } from "@/components/dashboard/dark-mode-loading";
import { useSearchParams } from "next/navigation"
import { useEffect } from "react";

export default function GoogleOAuthCallback() {
    const searchParams = useSearchParams();
    const code = searchParams.get("code");

    useEffect(() => {
        async function handleGoogleOAuth() {
            if (code && code.length > 0) {
                try {
                    let res = await access_google_oauth(code);
                    let data: Response<string> = res.data as Response<string>;

                    if(!(data.success && data.message && data.data)) return; // invalid response
                    if(data.data.length > 0) {
                        localStorage.setItem("access_token", data.data);
                    }
                } catch (error) {
                    console.error("Google OAuth Grindless API Error", error);
                    window.location.href = "/";
                }
            }
            
            window.location.href = "/launchpad";
        }

        handleGoogleOAuth()
    }, []);

    return (
        <DarkModeLoading />
    )
}