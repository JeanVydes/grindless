import { Button } from "@/components/ui/button";
import { Logout } from "@/providers/Auth";

export default function Settings() {
    return (
        <div className="flex flex-col items-center w-full h-auto space-y-4 py-4">
            <h1 className="text-4xl font-semibold text-foreground">Settings</h1>
            <Button variant={'outline'} onClick={Logout}>Logout</Button>
        </div>
    )
}