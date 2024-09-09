'use client'

import { GetOAuthGoogleURL } from "@/auth/google"
import { Button } from "@/components/ui/button"
import useOAuthStore from "@/stores/oauth"
import { MenuIcon, XIcon } from "lucide-react"
import Link from "next/link"
import { useEffect, useState } from "react"
import { GoogleIcon } from "./icons/google"

export default function LandingPage() {
    const [isMenuOpen, setIsMenuOpen] = useState(false)
    let oauthStore = useOAuthStore();

    function handleGoogleSignIn() {
        const google_oauth_url = GetOAuthGoogleURL({ date: new Date().toISOString() });
        window.location.href = google_oauth_url;
    }

    return (
        <div className="min-h-screen bg-gray-950 text-white overflow-hidden">
            {/* Abstract background with blur */}
            <div className="absolute inset-0 z-0">
                <div className="absolute inset-0 bg-gradient-to-br from-gray-900 via-gray-950 to-black opacity-70"></div>
                <div className="absolute inset-0 backdrop-blur-[50px]"></div>
                <div className="absolute inset-0">
                    <div className="h-full w-full bg-[radial-gradient(circle_500px_at_50%_200px,#1a1a1a,transparent)]"></div>
                </div>
            </div>

            {/* Content */}
            <div className="relative z-10 min-h-screen flex flex-col">
                {/* Header */}
                <header className="flex items-center justify-between p-4 md:p-6">
                    <Link href="/" className="text-2xl font-bold text-white">
                        Grindless
                    </Link>
                    <nav className="hidden md:flex space-x-6">
                        <Link href="#features" className="text-sm text-gray-300 hover:text-white transition-colors flex items-center justify-center">Features</Link>
                        <Link href="#pricing" className="text-sm text-gray-300 hover:text-white transition-colors flex items-center justify-center">Pricing</Link>
                        <Link href="#about" className="text-sm text-gray-300 hover:text-white transition-colors flex items-center justify-center">About</Link>
                        <Link href="#contact" className="text-sm text-gray-300 hover:text-white transition-colors flex items-center justify-center">Contact</Link>
                        {oauthStore.authenticated && <>
                            <img src={oauthStore.profile?.avatar || "/avatar.png"} className="w-8 h-8 rounded-full mr-2" alt="" referrerPolicy="no-referrer" />
                        </>}
                        {!oauthStore.authenticated && <>
                            <Button className="flex flex-row items-center justify-center space-x-2 text-white" variant="ghost" size="sm" onClick={() => {
                                handleGoogleSignIn();
                            }}>
                                <GoogleIcon className="h-6 w-6" />
                                <span>Sign in With Google</span>
                            </Button>
                        </>}
                    </nav>
                    <Button variant="ghost" size="icon" className="md:hidden z-50" onClick={() => setIsMenuOpen(!isMenuOpen)}>
                        {isMenuOpen ? <XIcon className="h-6 w-6" /> : <MenuIcon className="h-6 w-6" />}
                        <span className="sr-only">{isMenuOpen ? 'Close menu' : 'Open menu'}</span>
                    </Button>
                </header>

                {/* Mobile menu */}
                {isMenuOpen && (
                    <div className="fixed inset-0 z-40 bg-gray-950 bg-opacity-90 backdrop-blur-md md:hidden">
                        <nav className="flex flex-col items-center justify-center h-full space-y-8">
                            <Link href="#features" className="text-2xl font-medium" onClick={() => setIsMenuOpen(false)}>Features</Link>
                            <Link href="#pricing" className="text-2xl font-medium" onClick={() => setIsMenuOpen(false)}>Pricing</Link>
                            <Link href="#about" className="text-2xl font-medium" onClick={() => setIsMenuOpen(false)}>About</Link>
                            <Link href="#contact" className="text-2xl font-medium" onClick={() => setIsMenuOpen(false)}>Contact</Link>
                            {oauthStore.authenticated && <>
                                <Link href="#dashboard" className="text-2xl font-medium" onClick={() => setIsMenuOpen(false)}>
                                    <img src={oauthStore.profile?.avatar || "/avatar.png"} className="w-auto h-full rounded-full mr-2" alt="" />
                                </Link>
                            </>}
                            {!oauthStore.authenticated && <>
                                <Link href="#login" className="text-2xl font-medium" onClick={() => {
                                    setIsMenuOpen(false);
                                    handleGoogleSignIn();
                                }}>Sign in With Google</Link>
                            </>}
                        </nav>
                    </div>
                )}

                {/* Hero section */}
                <main className="flex-grow flex items-center justify-center px-4 py-16 text-center">
                    <div>
                        <h1 className="text-5xl md:text-7xl font-bold mb-6 bg-clip-text text-transparent bg-gradient-to-r from-blue-200 to-lime-400">
                            Less is More
                        </h1>
                        <p className="text-xl md:text-xl mb-8 max-w-2xl mx-auto text-gray-300">
                            Achieve more with less effort. Optimize your workflow and boost productivity without the burnout.
                        </p>
                        <div className="flex justify-center">
                            <Button onClick={() => {
                                if (oauthStore.authenticated) {
                                    window.location.href = "/launchpad"
                                    return;
                                }

                                handleGoogleSignIn();
                            }} className="w-full sm:w-auto bg-gradient-to-r from-lime-400 to-lime-700 text-white hover:from-lime-500 hover:to-lime-800 font-medium text-lg px-12 py-3 rounded-lg shadow-lg hover:shadow-xl transition-all duration-200">
                                {oauthStore.authenticated ? "Dashboard" : "Get Started"}
                            </Button>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    )
}
