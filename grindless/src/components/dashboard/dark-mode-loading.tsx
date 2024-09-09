'use client'

import { Loader2 } from "lucide-react"

export function DarkModeLoading() {
    return (
        <div className="fixed inset-0 flex items-center justify-center bg-black">
            <Loader2 className="h-16 w-16 animate-spin text-neutral-300" />
            <span className="sr-only">Loading...</span>
        </div>
    )
}