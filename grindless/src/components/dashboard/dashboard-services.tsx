"use client"

import { Card, CardContent } from "@/components/ui/card"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { BrushIcon, DatabaseIcon } from "@/components/dashboard/dashboard"
import { Sections } from "./sections/sections"
import { Eye } from "lucide-react"

export default function DashboardServices({ setSelectedSection }: {
    setSelectedSection: (section: Sections) => void,
}) {
    return (
        <main className="grid flex-1 items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8">
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
                {services.map((service, index) => (
                    <Card key={index} className="group relative overflow-hidden rounded-lg border bg-background shadow-sm transition-all hover:shadow-md hover:bg-muted">
                        <Link href="#summary" onClick={() => setSelectedSection(service.section)} className="absolute inset-0 z-10" prefetch={false}>
                            <span className="sr-only">View Service</span>
                        </Link>
                        <CardContent className="flex w-full h-full flex-col items-start justify-between gap-4 p-6">
                            <div className="flex items-center gap-4">
                                <div className="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
                                    {service.icon}
                                </div>
                                <div className="text-lg font-semibold">{service.name}</div>
                            </div>
                            <p className="text-sm text-muted-foreground">{service.description}</p>
                            <Button variant="outline" size="sm" className="w-full mt-auto">
                                <Eye className="h-5 w-5 mr-2" />
                                View
                            </Button>
                        </CardContent>
                    </Card>
                ))}
            </div>
        </main>
    )
}

export interface ServiceItem {
    id: string,
    name: string,
    description: string,
    to: string,
    section: Sections,
    icon: any,
    cost?: number,
}

export const services: ServiceItem[] = [
    {
        id: "summary",
        name: "Text Summarizer",
        description: "Effortlessly distill complex documents into concise, insightful summaries.",
        to: "/services/summary",
        section: Sections.ServiceSummaryText,
        icon: <BrushIcon className="h-5 w-5 text-muted-foreground" />,
        cost: 1,
    },
    {
        id: "scientific-paper",
        name: "Scientific Manuscript Creator",
        description: "Craft comprehensive scientific papers with precision and clarity.",
        to: "/services/scientific-paper",
        section: Sections.Main,
        icon: <DatabaseIcon className="h-5 w-5 text-muted-foreground" />,
        cost: 2,
    },
]