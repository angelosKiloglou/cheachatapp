'use client'

import React, {useEffect, useState} from 'react'
import {ScrollArea} from "@/components/ui/scroll-area"
import {Avatar, AvatarFallback} from "@/components/ui/avatar"
import ChatPanel from "@/components/chat-panel"
import {fetchUserId} from "@/lib/user-utils";

type ChatOverview = {
    chat_id: number;
    last_message: string;
    last_message_at: number;
    other_user: {
        id: number;
        email: string;
        first_name: string;
        last_name: string;
        username: string;
    };
}

export default function ChatList() {
    const [chats, setChats] = useState<ChatOverview[]>([])
    const [selectedChat, setSelectedChat] = useState<ChatOverview | null>(null)
    const [currentUserId, setCurrentUserId] = useState<number>(-1)

    useEffect(() => {
        // Fetch chats from the server
        const fetchChats = async () => {
            try {
                const chatsUrl = 'http://localhost:9000/get-chats';
                const response = await fetch(chatsUrl, {
                    method: 'GET',
                    headers: { 'Content-Type': 'application/json' },
                    credentials: 'include'

                });
                if (response.ok) {
                    const data = await response.json()
                    console.log(chats)
                    setChats(data)
                }
            } catch (error) {
                console.error('Error fetching chats:', error)
            }
        }

        // Fetch current user ID
        const fetchCurrentUserId = async () => {
            // Replace this with your actual method to fetch the current user ID
            const userId = await fetchUserId();
            setCurrentUserId(userId)
        }

        fetchChats()
        fetchCurrentUserId()
    }, [])

    const handleChatSelect = (chat: ChatOverview) => {
        handleCloseChat()
        setSelectedChat(chat)
    }

    const handleCloseChat = () => {
        setSelectedChat(null)
    }

    return (
        <div className="right-0 top-0 bottom-0 flex h-full">
            <div className="w-80 bg-[#44443c] border-l border-[#55554d]">
                <h2 className="p-4 text-xl font-semibold text-white bg-[#55554d]">Chats</h2>
                <ScrollArea className="h-[calc(100vh-64px)]">
                    {chats.map((chat) => (
                        <div
                            key={chat.chat_id}
                            className="p-4 border-b border-[#55554d] hover:bg-[#55554d] cursor-pointer"
                            onClick={() => handleChatSelect(chat)}
                        >
                            <div className="flex items-center space-x-4">
                                <Avatar>
                                    <AvatarFallback className="bg-[#66665b] text-white">
                                        {chat.other_user.first_name[0]}{chat.other_user.last_name[0]}
                                    </AvatarFallback>
                                </Avatar>
                                <div className="flex-1 min-w-0">
                                    <p className="text-sm font-medium text-white truncate">
                                        {chat.other_user.username}
                                    </p>
                                    <p className="text-sm text-gray-300 truncate">
                                        {chat.last_message}
                                    </p>
                                </div>
                                <div className="text-xs text-gray-400">
                                    {new Date(chat.last_message_at * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                                </div>
                            </div>
                        </div>
                    ))}
                </ScrollArea>
            </div>
            {selectedChat && (
                <ChatPanel
                    chat_id={selectedChat.chat_id}
                    current_user_id={currentUserId}
                    otherUser={selectedChat.other_user}
                    onClose={handleCloseChat}
                />
            )}
        </div>
    )
}