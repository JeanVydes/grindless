"use client"

import Link from "next/link"
import { Sheet, SheetTrigger, SheetContent } from "@/components/ui/sheet"
import { Button } from "@/components/ui/button"
import { Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator, BreadcrumbPage } from "@/components/ui/breadcrumb"
import { Input } from "@/components/ui/input"
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuItem } from "@/components/ui/dropdown-menu"
import { BriefcaseIcon, HomeIcon, LineChartIcon, MenuIcon, SearchIcon, SettingsIcon, UsersIcon } from "./dashboard"
import { OAuthStore } from "@/stores/oauth"
import { Sections } from "./sections/sections"
import { Fragment, useEffect, useState } from "react"
import { ServiceItem, services } from "./dashboard-services"
import { Popover, PopoverContent, PopoverTrigger } from "@radix-ui/react-popover"
import { Check, ChevronsUpDown, CreditCard, Settings, Smile } from "lucide-react"
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandSeparator } from "cmdk"
import { CommandShortcut } from "../ui/command"
import { Sarpanch } from "next/font/google"
import { Logout } from "@/providers/Auth"

export const ServicesBreadcrumb = (setSelectedSection: (section: Sections) => void) => {
    return (
        <BreadcrumbItem className="hover:cursor-pointer" onClick={() => setSelectedSection(Sections.Main)}>
            <BreadcrumbPage>Services</BreadcrumbPage>
        </BreadcrumbItem>
    )
}

export const CreditsBreadcrumb = (setSelectedSection: (section: Sections) => void) => {
    return (
        <BreadcrumbItem className="hover:cursor-pointer" onClick={() => setSelectedSection(Sections.Credits)}>
            <BreadcrumbPage>Credits</BreadcrumbPage>
        </BreadcrumbItem>
    )
}

export const ServiceBreadcrumb = (setSelectedSection: (section: Sections) => void, service: ServiceItem) => {
    return (
        <BreadcrumbItem className="hover:cursor-pointer" onClick={() => setSelectedSection(service.section)}>
            <BreadcrumbPage>{service.name}</BreadcrumbPage>
        </BreadcrumbItem>
    )
}

export const SettingsBreadcrumb = (setSelectedSection: (section: Sections) => void) => {
    return (
        <BreadcrumbItem className="hover:cursor-pointer" onClick={() => setSelectedSection(Sections.Settings)}>
            <BreadcrumbPage>Settings</BreadcrumbPage>
        </BreadcrumbItem>
    )
}

export const LimboBreadcrumb = (setSelectedSection: (section: Sections) => void) => {
    return (
        <BreadcrumbItem className="hover:cursor-pointer" onClick={() => setSelectedSection(Sections.Main)}>
            <BreadcrumbPage>Limbo</BreadcrumbPage>
        </BreadcrumbItem>
    )
}

export default function DashboardTopBar({ oauthStore, selectedSection, setSelectedSection }: {
    oauthStore: OAuthStore,
    selectedSection: Sections,
    setSelectedSection: (section: Sections) => void,
}) {
    let [breadcrumbs, setBreadcrumbs] = useState<any[]>([])
    let [search, setSearch] = useState("")
    let [searchResults, setSearchResults] = useState<any[]>([])
    let [open, setOpen] = useState(false)

    function createBreadcrumbs(section: Sections) {
        switch (section) {
            case Sections.Main:
                setBreadcrumbs([])
                return
            case Sections.Credits:
                setBreadcrumbs([CreditsBreadcrumb(setSelectedSection)])
                return
            case Sections.ServiceSummaryText:
                setBreadcrumbs([ServicesBreadcrumb(setSelectedSection), <BreadcrumbSeparator />, ServiceBreadcrumb(setSelectedSection, services.find(service => service.id === "summary")!)])
                return
            case Sections.Settings:
                setBreadcrumbs([SettingsBreadcrumb(setSelectedSection)])
                return
            default:
                setBreadcrumbs([LimboBreadcrumb(setSelectedSection)])
                return
        }
    }

    function onSearchChange(query: any) {
        setSearch(query);

        if (query.length > 0) {
            const filteredServices = services.filter(service =>
                service.name.toLowerCase().includes(query.toLowerCase())
            );
            setSearchResults(filteredServices);
        } else {
            setSearchResults([]);
        }
    }

    function localSetSelectedSection(section: Sections) {
        console.log("Setting section (top bar)", section)
        setSelectedSection(section);
        setOpen(false);
    }

    useEffect(() => {
        createBreadcrumbs(selectedSection)
    }, [selectedSection]);

    return (
        <header className="w-full sticky top-0 z-30 flex flex-row justify-between h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6">
            <Sheet>
                <SheetTrigger asChild>
                    <Button size="icon" variant="outline" className="sm:hidden">
                        <MenuIcon className="h-5 w-5" />
                        <span className="sr-only">Toggle Menu</span>
                    </Button>
                </SheetTrigger>
                <SheetContent side="left" className="sm:max-w-xs">
                    <nav className="grid gap-6 text-lg font-medium">
                        <Button variant="ghost" size="sm"
                            onClick={() => setSelectedSection(Sections.Main)}
                            className="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
                        >
                            <HomeIcon className="h-5 w-5" />
                            Launchpad
                        </Button>
                        <Button variant="ghost" size="sm"
                            onClick={() => setSelectedSection(Sections.Credits)}
                            className="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"    
                        >
                            <BriefcaseIcon className="h-5 w-5" />
                            Credits
                        </Button>
                        <Button variant="ghost" size="sm"
                            onClick={() => setSelectedSection(Sections.Settings)}
                            className="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
                        >
                            <UsersIcon className="h-5 w-5" />
                            Settings
                        </Button>
                    </nav>
                </SheetContent>
            </Sheet>
            <Breadcrumb className="hidden md:flex">
                <BreadcrumbList>
                    <BreadcrumbItem>
                        <BreadcrumbPage>Grindless</BreadcrumbPage>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator />
                    <BreadcrumbItem>
                        {breadcrumbs.map((breadcrumb, index) => (
                            <Fragment key={index}>{breadcrumb}</Fragment>
                        ))}
                    </BreadcrumbItem>
                </BreadcrumbList>
            </Breadcrumb>
            <div className="flex flex-row items-center justify-around space-x-4">
                <div className="relative">
                    <Command className="w-full relative rounded-xl border border-muted bg-background">
                        
                        <CommandInput
                            className="flex-1 px-4 py-2 bg-background border border-muted rounded-xl"
                            placeholder="Search services..."
                            onValueChange={onSearchChange}
                            onFocus={() => setOpen(true)}
                            onBlur={() => setOpen(false)}
                        />
                        {open && (
                            <CommandList className="absolute bg-background/50 backdrop-blur-md rounded-b-xl border border-muted">
                                <CommandEmpty>No results found.</CommandEmpty>
                                <CommandGroup>
                                    {searchResults.map((service: ServiceItem, index) => {
                                        return (
                                            <CommandItem className="hover:cursor-pointer hover:bg-muted px-4 py-4 flex flex-row items-center justify-left text-left space-x-2" key={index} value={service.name} onSelect={() => console.log("Shit")} onClick={() => localSetSelectedSection(service.section)}>
                                                {service.icon}
                                                <span>{service.name}</span>
                                            </CommandItem>
                                        )
                                    })}
                                    {search.length === 0 && services.map((service: ServiceItem, index) => {
                                        return (
                                            <CommandItem className="hover:cursor-pointer hover:bg-muted px-4 py-4 flex flex-row items-center justify-left text-left space-x-2" key={index} value={service.name} onSelect={() => console.log("Shit")} onClick={() => localSetSelectedSection(service.section)}>
                                                {service.icon}
                                                <span>{service.name}</span>
                                            </CommandItem>
                                        )
                                    })}
                                </CommandGroup>
                            </CommandList>
                        )}
                    </Command>
                </div>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Button variant="outline" size="icon" className="overflow-hidden rounded-full">
                            <img
                                src={oauthStore.profile?.avatar || "https://upload.wikimedia.org/wikipedia/commons/7/7c/Profile_avatar_placeholder_large.png?20150327203541"}
                                width={36}
                                height={36}
                                alt="Avatar"
                                className="overflow-hidden rounded-full"
                                style={{ aspectRatio: "36/36", objectFit: "cover" }}
                            />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent className="bg-background py-2 mt-2" align="end">
                        <DropdownMenuLabel className="px-12 py-2">My Account</DropdownMenuLabel>
                        <DropdownMenuItem>
                            <CreditCard className="h-5 w-5 mr-2" />
                            {oauthStore.billing?.credits || 0} Credits
                        </DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem className="hover:bg-muted" onClick={() => {
                            setSelectedSection(Sections.Settings)
                        }}>Settings</DropdownMenuItem>
                        <DropdownMenuItem disabled className="hover:cursor-pointer hover:bg-muted">Support</DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem className="hover:cursor-pointer"onClick={Logout}>
                            Logout
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>
        </header>
    )
}