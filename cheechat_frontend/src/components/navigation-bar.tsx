'use client'

import React, {useEffect, useState} from 'react'
import Link from 'next/link'
import {LogOut, Search, User} from 'lucide-react'
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger} from "@/components/ui/dropdown-menu"
import {Avatar, AvatarFallback} from "@/components/ui/avatar"
import {Card, CardContent} from "@/components/ui/card"
import ChatPanel from "@/components/chat-panel"
import {fetchUserId} from "@/lib/user-utils"
import Image from 'next/image'

type UserDetails = {
    id: number;
    username: string;
    email: string;
    first_name: string;
    last_name: string;
}

export default function NavigationBar() {
    const [searchQuery, setSearchQuery] = useState('')
    const [searchResult, setSearchResult] = useState<UserDetails | null>(null)
    const [chatId, setChatId] = useState<number | null>(null)
    const [currentUserId, setCurrentUserId] = useState<number>(-1)

    useEffect(() => {
        const getUserId = async () => {
            const userId = await fetchUserId();
            setCurrentUserId(userId);
        };

        getUserId();
    }, []);

    const handleSearch = async (e: React.FormEvent) => {
        e.preventDefault()
        try {
            const getUserUrl = `http://localhost:9000/get-user?username=${searchQuery}`
            const response = await fetch(getUserUrl, {
                method: 'GET',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include'
            })
            if (response.ok) {
                const data = await response.json()
                setSearchResult(data)
            } else {
                setSearchResult(null)
            }
        } catch (error) {
            console.error('Error searching for user:', error)
            setSearchResult(null)
        }
    }

    const handleSearchResultClick = async () => {
        if (searchResult) {
            try {
                const userChatUrl = `http://localhost:9000/chats`
                const response = await fetch(userChatUrl, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ recipient: searchResult.username }),
                    credentials: 'include'
                })
                if (response.ok) {
                    const chatId = await response.json()
                    setChatId(chatId)
                    console.log('User details action performed:', chatId)
                }
            } catch (error) {
                console.error('Error performing action on user details:', error)
            }
        }
    }

    const logoutUrl = 'http://localhost:9000/logout';
    const handleLogout = async () => {
        await fetch(logoutUrl, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include'
        })
        window.location.href = '/login'
    }

    return (
        <>
            <nav className="flex items-center justify-between p-4 bg-[#44443c] text-white">
                <Link href="/dashboard" className="flex items-center">
                    <div className="rounded-full overflow-hidden w-12 h-12 bg-white">
                        <Image
                            src="/logo.png"
                            alt="CheeChat Logo"
                            width={48}
                            height={48}
                            priority
                            className="object-cover"
                        />
                    </div>
                </Link>

                <form onSubmit={handleSearch} className="flex-1 max-w-md mx-4">
                    <div className="relative">
                        <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 text-[#44443c]" />
                        <Input
                            type="search"
                            placeholder="Search for a user..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="w-full pl-8 bg-white text-[#44443c] placeholder-[#44443c]/50"
                        />
                    </div>
                </form>

                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="icon" className="rounded-full text-white hover:bg-[#55554d]">
                            <User className="h-5 w-5" />
                            <span className="sr-only">User menu</span>
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="bg-[#44443c] text-white">
                        <DropdownMenuItem onClick={handleLogout} className="hover:bg-[#55554d]">
                            <LogOut className="mr-2 h-4 w-4" />
                            <span>Log out</span>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </nav>

            {searchResult && (
                <Card className="mt-4 mx-auto max-w-md" onClick={handleSearchResultClick}>
                    <CardContent className="flex items-center space-x-4 p-4 cursor-pointer">
                        <Avatar>
                            <AvatarFallback>{searchResult.first_name[0]}{searchResult.last_name[0]}</AvatarFallback>
                        </Avatar>
                        <div>
                            <h3 className="font-semibold">{searchResult.username}</h3>
                            <p className="text-sm text-muted-foreground">{searchResult.email}</p>
                            <p className="text-sm">{searchResult.first_name} {searchResult.last_name}</p>
                        </div>
                    </CardContent>
                </Card>
            )}

            {chatId && searchResult && (
                <ChatPanel chat_id={chatId} current_user_id={currentUserId} otherUser={searchResult} onClose={() => setChatId(null)} />
            )}
        </>
    )
}