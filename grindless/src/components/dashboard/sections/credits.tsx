import { Button } from "@/components/ui/button";

export interface PlanItem {
    title: string,
    credits: number | string,
    price?: number,
    description: string,
}

export const plans: PlanItem[] = [
    {
        title: "Rookie",
        credits: 200,
        price: 5.99,
        description: "Get started with 200 credits, perfect for simple tasks.",
    },
    {
        title: "Pro",
        credits: 2000,
        price: 49.99,
        description: "Upgrade to 2000 credits, unlock the full potential.",
    },
    {
        title: "Custom",
        credits: "Custom",
        price: 0.03,
        description: "Buy a custom amount of credits.",
    },
]

export default function Credits() {

    return (
        <div className="flex flex-col items-center w-full h-screen space-y-4 py-4">
            <h1 className="text-4xl font-semibold text-foreground">Credits</h1>
            <div className="flex flex-col flex-1 md:flex-row lg:flex-row items-center w-full h-auto space-y-4 py-4 px-4">
                {plans.map((plan, index) => (
                    <PlanCard key={index} {...plan} />
                ))}
            </div>
        </div>
    )
}

export function PlanCard({ title, credits, price, description }: any) {
    return (
        <div className={`
            max-h-screen w-full 
            flex-1 flex flex-col items-center justify-center
            border ${title == "Pro" ? "border-lime-400" : "border-muted"} rounded-xl
            bg-background shadow-sm transition-all hover:shadow-md hover:bg-muted hover:cursor-pointer
        `}>
            <h2 className={`text-2xl font-semibold ${title == "Pro" ? "text-lime-400" : "text-foreground"}`}>{title}</h2>
            <p>{credits} Credits</p>
            <div className="text-sm text-muted flex flex-row items-center justify-center space-x-2">
                {title == "Rookie" && (
                    <>
                        <p className="text-foreground"
                        >{price} USD</p>
                    </>
                )}

                {title == "Pro" && (
                    <>
                        <p className="text-foreground"
                        >{price} USD</p>
                        <p className="text-sm text-lime-400 font-semibold">18% Discount</p>
                    </>
                )}

                {title == "Custom" && (
                    <div className="flex flex-row items-center justify-center space-x-1">
                        <p className="text-foreground"
                        >{price} USD per credit</p>
                    </div>
                )}
            </div>
            <p className="text-sm">{description}</p>
            {title == "Pro" && (
                <p className="text-red-100 font-semibold"
                >Most loved ❤️</p>
            )}
        </div>
    )
}