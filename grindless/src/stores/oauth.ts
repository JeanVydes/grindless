import { Account } from '@/api/entities/account';
import { Billing } from '@/api/entities/billing';
import { create } from 'zustand';

export interface OAuthStore {
    authenticated: boolean,
    checked: boolean,
    access_token: string | null,
    profile: Account | undefined | null,
    billing: Billing | undefined | null,
}

export const useOAuthStore = create<OAuthStore>((set) => ({
    authenticated: false,
    checked: false,
    access_token: null,
    profile: null,
    billing: null,
}));

export default useOAuthStore;