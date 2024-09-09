import type { Metadata } from "next";
import { Montserrat } from "next/font/google";
import "./globals.css";
import AuthProvider from "@/providers/Auth";

const font = Montserrat({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Grindless",
    description: "Less is more",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <AuthProvider>
            <html lang="en">
                <body className={`w-full h-auto p-0 m-0 ${font.className}`}>{children}</body>
            </html>
        </AuthProvider>
    );
}
