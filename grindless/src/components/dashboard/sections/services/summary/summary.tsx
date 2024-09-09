import { useEffect, useState } from 'react'
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { summary_api } from '@/api/services'
import useOAuthStore, { OAuthStore } from '@/stores/oauth'
import { Response } from '@/api/api'
import { BookOpenCheck, ClipboardCopy, Eraser, Loader2, WandSparkles } from 'lucide-react'
import { services } from '@/components/dashboard/dashboard-services'
import { UpdateCacheBilling } from '@/providers/Auth'

//import * as pdfjsLib from 'pdfjs-dist/webpack.mjs';
//import * as pdfjsWorker from 'pdfjs-dist/build/pdf.worker.min.mjs';

import * as PDFJS from "pdfjs-dist/types/src/pdf";
import { CalculatePricing, SUMMARY_1000_TOKENS_COST } from '@/api/util'
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select'

export const summary_kind = ["plain", "json", "html", "markdown_table"];
export type SummaryKind = typeof summary_kind[number];

export function summary_kind_label(kind: SummaryKind): string {
    switch (kind) {
        case "plain":
            return "Plain Text";
        case "json":
            return "JSON";
        case "html":
            return "HTML";
        case "markdown_table":
            return "Markdown";
    }
    return "";
}

export default function SummaryService({ oauthStore }: {
    oauthStore: OAuthStore
}) {
    const [chatContent, setChatContent] = useState('')
    const [summary, setSummary] = useState('')
    const [error, setError] = useState('')
    const [loading, setLoading] = useState(false)
    const [file, setFile] = useState<File | null>(null)
    const [pdfjs, setPDFJS] = useState<typeof PDFJS | null>(null);
    const [summaryKind, setSummaryKind] = useState<SummaryKind>("plain");

    // 32768 Tokens * 4 (token length) = 131072
    const maxCharacters = 131072;
    const thisService = services.find(service => service.id === "summary")
    const thisPricing = CalculatePricing(SUMMARY_1000_TOKENS_COST, chatContent.length);

    if (!thisService) return null

    // Load the PDFJS library once on component mount
    useEffect(() => {
        import("pdfjs-dist/webpack.mjs").then(setPDFJS)
    }, []);

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const input = e.target.value
        if (input.length <= maxCharacters) {
            setChatContent(input)
        }

        let trimmed = input.substring(0, maxCharacters)
        if (trimmed !== input) {
            setChatContent(trimmed)
        }
    }

    const generateSummary = async () => {
        if (loading) return

        setLoading(true)
        setError('')

        if (chatContent.length === 0) {
            setError("Please enter some text to summarize.")
            setLoading(false)
            return
        }

        if (chatContent.length < 10) {
            setError("Please enter at least 10 characters to summarize.")
            setLoading(false)
            return
        }

        if (chatContent.length > maxCharacters) {
            setError(`Please enter less than ${maxCharacters} characters to summarize.`)
            setLoading(false)
            return
        }

        if (!oauthStore.access_token || !oauthStore.authenticated || !oauthStore.checked) {
            setError("You are not authenticated.")
            setLoading(false)
            return
        }

        if (oauthStore.billing && oauthStore.billing.credits < thisPricing) {
            setError("Insufficient credits to perform this operation. Buy more")
            setLoading(false)
            return
        }

        try {
            if (!oauthStore.access_token) return;
            let res = await summary_api(oauthStore.access_token, summaryKind, chatContent);
            let data = res.data;
            if (!(data.success && data.message && data.data)) return; // invalid response

            if (data.success == "error") {
                setError(data.message)
                setLoading(false)
                return
            }

            setSummary(data.data.message)
            setChatContent("")

            let billing = oauthStore.billing
            if (billing) {
                billing.credits = data.data.remaining_credits
                useOAuthStore.setState({ billing })
                UpdateCacheBilling(billing)
            }

            setLoading(false)
        } catch (err) {
            console.error('PDF extraction error:', err);
            setError('Failed to summarize text');
            setLoading(false);
        }
    }

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files && e.target.files.length > 0) {
            let file = e.target.files[0]
            setFile(file)
            extractTextFromPDF(file)
        }
    }

    const extractTextFromPDF = async (pdfFile: File) => {
        if (!pdfjs) {
            setError('PDF.js library is not loaded.');
            return;
        }

        try {
            const arrayBuffer = await pdfFile.arrayBuffer();
            const pdf = await pdfjs.getDocument({ data: arrayBuffer }).promise;
            let fullText = '';

            for (let i = 1; i <= pdf.numPages; i++) {
                const page = await pdf.getPage(i);
                const textContent = await page.getTextContent();
                //@ts-ignore
                const pageText = textContent.items.map(item => item.str).join(' ');
                fullText += pageText + '\n';
            }

            console.log("PDF Extracted, Length", fullText.length);

            fullText = fullText.substring(0, maxCharacters);
            setChatContent(fullText);
        } catch (err) {
            console.error('PDF extraction error:', err);
            setError('Failed to extract text');
            setLoading(false);
        }
    };

    const onKindChange = (kind: SummaryKind) => {
        setSummaryKind(kind);
    }

    return (
        <div className="w-full flex flex-col items-center justify-center pt-8 overflow-auto">
            <Card className='bg-background md:w-2/3 lg:1/2 sm:w-full py-4'>
                <CardHeader>
                    <CardTitle className="flex flex-row items-center space-x-2">
                        <BookOpenCheck className="mr-2 h-6 w-6" />
                        Summary AI
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="space-y-4">
                        <div>
                            <label htmlFor="chat-input" className="block text-foreground text-muted pb-2">
                                Enter the text you want to summarize
                            </label>
                            <Textarea
                                id="chat-input"
                                value={chatContent}
                                onChange={handleInputChange}
                                placeholder="Paste your text here..."
                                className="w-full h-40"
                                disabled={loading}
                            />
                            <div className="w-full flex flex-col justify-between items-center pt-4 space-y-2">
                                <div className="w-full flex flex-row justify-between items-center space-x-2">
                                    <p className="text-sm text-muted">
                                        {chatContent.length}/{maxCharacters} characters
                                    </p>
                                    <p className="text-sm text-muted">
                                        Operation Price in Credits: {thisPricing}
                                    </p>
                                    <Button disabled={loading} variant="outline" size="sm" className={`px-8 ${loading ? "hover:cursor-wait bg-muted" : ""}`} onClick={() => {
                                        setChatContent('')
                                    }}>
                                        <Eraser className="mr-2 h-4 w-4" />
                                        Clear
                                    </Button>
                                </div>
                                <div className="w-full flex flex-row justify-between items-start space-x-4 h-16">
                                    {error && (<>
                                        <span className="text-red-500">{error}</span>
                                    </>)}

                                    <div className='flex-1 flex flex-col justify-center items-center space-y-1'>
                                        <input id="fileUpload" className="w-full flex flex-row items-center justify-center m-auto hover:cursor-pointer w-1/2 h-full text-center placeholder-gray-200 text-sm text-foreground border border-muted rounded-xl p-2 file:bg-gray-50
                                        file:border-0
                                        file:me-4
                                        file:py-2 file:px-4
                                        file:rounded-xl
                                        file:w-1/2
                                        "
                                            type="file" accept="application/pdf" onChange={handleFileChange} />
                                        <label className="w-full text-muted text-sm" htmlFor="fileUpload">Only support PDF</label>
                                    </div>
                                    <Select onValueChange={onKindChange} defaultValue="plain">
                                        <SelectTrigger className="flex-1 h-16">
                                            <SelectValue placeholder="Select a fruit" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectGroup>
                                                {summary_kind.map((kind) => (
                                                    <SelectItem value={kind} key={kind} onClick={() => setSummaryKind(kind)}>
                                                        {summary_kind_label(kind)}
                                                    </SelectItem>
                                                ))}
                                            </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                </div>
                            </div>
                        </div>
                        <Button onClick={generateSummary} className="w-full">
                            {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                            {!loading && <WandSparkles className="mr-2 h-4 w-4" />}
                            Generate Summary
                        </Button>
                        {summary && (
                            <div className="flex flex-col items-center justify-center space-y-2">
                                {loading && <p>Summarizing your text...</p>}
                                <div className="border rounded-xl border-white p-4 rounded-md whitespace-pre-line">{summary}</div>
                                <div className="flex flex-row items-center justify-center space-x-2">
                                    <Button className="w-full" onClick={() => {
                                        navigator.clipboard.writeText(summary);
                                        alert("Text copied! :)")
                                    }}>
                                        <ClipboardCopy className="mr-2 h-4 w-4" />
                                        Copy
                                    </Button>
                                </div>
                            </div>
                        )}
                    </div>
                </CardContent>
            </Card>
        </div>
    )
}
