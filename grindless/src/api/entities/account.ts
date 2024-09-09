export interface Account {
    id: number,
    account_id: string,
    email: string,
    username: string,
    name: string,
    avatar: string | undefined | null,
    flags: number[],
    created_at: number,
    updated_at: number,
    deleted: boolean,
    deletion_requested_at: number | undefined | null,
    deletion_reason: string | undefined | null,
}