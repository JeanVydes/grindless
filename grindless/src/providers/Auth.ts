"use client"

import { get_my_info } from "@/api/accounts";
import { Response } from "@/api/api";
import { Account } from "@/api/entities/account";
import { Billing } from "@/api/entities/billing";
import useOAuthStore from "@/stores/oauth";
import { useEffect } from "react";

export default function AuthProvider({ children }: {
    children: React.ReactNode
}) {
    async function fetch_my_account(access_token: string): Promise<{
        account: Account,
        billing: Billing
    }> {
        let res = await get_my_info(access_token || "");

        let data: Response<{
            account: Account,
            billing: Billing
        }> = res.data as Response<{
            account: Account,
            billing: Billing
        }>;

        if(!(data.success && data.message && data.data && data.data.account && data.data.billing)) return Promise.reject("Invalid response");

        if (data.success == "error") {
            return Promise.reject(data.message);
        }

        if(data.data.account.id > 0) {
            let user = JSON.stringify(data.data.account);
            localStorage.setItem("@me", user);
            localStorage.setItem("@me.billing", JSON.stringify(data.data.billing));
            localStorage.setItem("@me.last_updated", Date.now().toString());
            useOAuthStore.setState({ authenticated: true, profile: data.data.account, billing: data.data.billing, access_token });
            return Promise.resolve(data.data);
        }

        return Promise.reject("Invalid account");
    }

    useEffect(() => {
        let access_token = localStorage.getItem("access_token");
        let last_updated = localStorage.getItem("@me.last_updated");
        let last_updated_obj = last_updated ? parseInt(last_updated) : 0;

        let profile_obj: Account | null = GetCacheProfile();
        let billing_obj: Billing | null = GetCacheBilling();

        // Profile exists and was updated within 5 minutes
        if (billing_obj && profile_obj && last_updated_obj && (Date.now() - last_updated_obj) < 300000) {
            console.log("Using profile cache");
            useOAuthStore.setState({ authenticated: true, checked: true, profile: profile_obj, billing: billing_obj, access_token });
            return;
        }

        if (access_token && access_token.length > 0) {
            try {
                fetch_my_account(access_token);
            } catch (error) {
                localStorage.removeItem("access_token");
                localStorage.removeItem("@me");
                localStorage.removeItem("@me.billing");
                localStorage.removeItem("@me.last_updated");
                useOAuthStore.setState({ checked: true });
            }
        }

        useOAuthStore.setState({ checked: true });
    }, []);

    return children;
}

export function GetCacheProfile(): Account | null {
    let profile = localStorage.getItem("@me");
    return profile ? JSON.parse(profile) : null;
}

export function GetCacheBilling(): Billing | null {
    let billing = localStorage.getItem("@me.billing");
    return billing ? JSON.parse(billing) : null;
}

export function UpdateCacheBilling(billing: Billing) {
    localStorage.setItem("@me.billing", JSON.stringify(billing));
    useOAuthStore.setState({ billing });
}

export function Logout() {
    localStorage.removeItem("access_token");
    localStorage.removeItem("@me");
    localStorage.removeItem("@me.billing");
    localStorage.removeItem("@me.last_updated");
    useOAuthStore.setState({ authenticated: false, profile: null, billing: null, access_token: "" });
    window.location.href = "/";
}