"use client"

import Link from "next/link"
import { BriefcaseIcon, CloudLightningIcon, HomeIcon, LineChartIcon, SettingsIcon, UsersIcon } from "@/components/dashboard/dashboard"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import { Sections } from "./sections/sections"

export default function DashboardMenu({
    setSelectedSection,
}: {
    setSelectedSection: (sections: Sections) => void
}) {
    return (
        <aside className="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background sm:flex">
            <nav className="flex flex-col items-center gap-4 px-2 sm:py-5">
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Link
                                href="#home"
                                onClick={() => setSelectedSection(Sections.Main)}
                                className="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
                                prefetch={false}
                            >
                                <HomeIcon className="h-5 w-5" />
                                <span className="sr-only">Dashboard</span>
                            </Link>
                        </TooltipTrigger>
                        <TooltipContent side="right">Dashboard</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Link
                                href="#credits"
                                onClick={() => setSelectedSection(Sections.Credits)}
                                className="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
                                prefetch={false}
                            >
                                <CloudLightningIcon className="h-5 w-5" />
                                <span className="sr-only">Credits</span>
                            </Link>
                        </TooltipTrigger>
                        <TooltipContent side="right">Credits</TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            </nav>
            <nav className="mt-auto flex flex-col items-center gap-4 px-2 sm:py-5">
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Link
                                href="#settings"
                                onClick={() => setSelectedSection(Sections.Settings)}
                                className="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
                                prefetch={false}
                            >
                                <SettingsIcon className="h-5 w-5" />
                                <span className="sr-only">Settings</span>
                            </Link>
                        </TooltipTrigger>
                        <TooltipContent side="right">Settings</TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            </nav>
        </aside>
    )
}