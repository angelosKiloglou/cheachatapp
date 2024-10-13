'use client'

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {useEffect} from "react";

export default function DashboardPage() {
    const logoutUrl = 'http://localhost:9000/logout';
    const handleLogout = async () => {
        await fetch(logoutUrl, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include'
        })
        window.location.href = '/login'
    }

    // useEffect(() => {
    //     const indexUrl = 'http://localhost:9000/';
    //     const index = async () => {
    //         await fetch(indexUrl, {
    //             method: 'GET',
    //             headers: { 'Content-Type': 'application/json' },
    //             credentials: 'include'
    //         })
    //     }
    //     index()
    // })

    return (
        <div className="flex items-center justify-center min-h-screen bg-background">
            <Card className="w-full max-w-md">
                <CardHeader>
                    <CardTitle className="text-2xl font-bold text-center">Dashboard</CardTitle>
                </CardHeader>
                <CardContent>
                    <p className="text-center mb-4">Welcome to your dashboard!</p>
                    <Button onClick={handleLogout} className="w-full">
                        Logout
                    </Button>
                </CardContent>
            </Card>
        </div>
    )
}