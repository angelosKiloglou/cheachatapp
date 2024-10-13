'use client'

import React, { useState, useEffect, useRef } from 'react'
import { X, Send, User } from 'lucide-react'
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { v4 as uuidv4 } from 'uuid';

type ChatPanelProps = {
    chat_id: number,
    current_user_id: number,
    otherUser?: {
        id: number;
        username: string;
        email: string;
        first_name: string;
        last_name: string;
    };
    onClose: () => void;
}

type Message = {
    id: string;
    sender: 'user' | 'other';
    content: string;
    timestamp: number;
}

const generateMessageId = (): string => {
    return uuidv4();
};

export default function ChatPanel({ chat_id, current_user_id, otherUser, onClose}: ChatPanelProps) {
    const [messages, setMessages] = useState<Message[]>([])
    const [newMessage, setNewMessage] = useState('')
    const [wsReady, setWsReady] = useState(false)
    const ws = useRef<WebSocket | null>(null)
    const scrollAreaRef = useRef<HTMLDivElement>(null)

    useEffect(() => {
        const chatUrl = `ws://localhost:9000/ws/chat/${chat_id}`;
        ws.current = new WebSocket(chatUrl)

        ws.current.onopen = () => {
            console.log('WebSocket connection established')
            setWsReady(true)
        }

        ws.current.onmessage = (event) => {
            const data = JSON.parse(event.data)
            const message: Message = {
                id: generateMessageId(),
                sender: data.sender_id === current_user_id ? 'user' : 'other',
                content: data.message,
                timestamp: new Date(data.sent_at).getTime()
            }
            setMessages((prevMessages) => [...prevMessages, message])
        }

        ws.current.onerror = (error) => {
            console.error('WebSocket error:', error)
        }

        ws.current.onclose = () => {
            console.log('WebSocket connection closed')
            setWsReady(false)
        }

        return () => {
            if (ws.current) {
                ws.current.close()
            }
        }
    }, [chat_id, current_user_id])

    useEffect(() => {
        if (scrollAreaRef.current) {
            scrollAreaRef.current.scrollTop = scrollAreaRef.current.scrollHeight
        }
    }, [messages])

    const handleSendMessage = (e: React.FormEvent) => {
        e.preventDefault()
        if (newMessage.trim() && wsReady && ws.current) {
            const message: Message = {
                id: generateMessageId(),
                sender: 'user',
                content: newMessage,
                timestamp: Date.now()
            }
            setMessages((prevMessages) => [...prevMessages, message])
            ws.current.send(newMessage)
            setNewMessage('')
        }
    }

    const handleClose = () => {
        onClose();
        ws.current?.close();
    }

    const otherUserInitials = otherUser
        ? `${otherUser.first_name[0] || ''}${otherUser.last_name[0] || ''}`.toUpperCase()
        : '';

    const otherUserName = otherUser?.username || 'Chat';

    return (
        <div className="fixed bottom-0 right-0 w-96 h-[32rem] bg-[#44443c] border border-[#55554d] rounded-tl-lg shadow-lg flex flex-col overflow-hidden">
            <div className="flex justify-between items-center p-4 border-b border-[#55554d] bg-[#55554d]">
                <div className="flex items-center">
                    <Avatar>
                        <AvatarFallback className="bg-[#66665b] text-white">
                            {otherUserInitials || <User className="h-6 w-6" />}
                        </AvatarFallback>
                    </Avatar>
                    <span className="ml-2 font-semibold text-white">{otherUserName}</span>
                </div>
                <Button variant="ghost" size="icon" onClick={handleClose} className="text-white hover:bg-[#66665b]">
                    <X className="h-5 w-5" />
                </Button>
            </div>
            <ScrollArea className="flex-grow bg-[#44443c]">
                <div ref={scrollAreaRef} className="p-4 space-y-4">
                    {messages.map((message) => (
                        <div key={message.id} className={`flex ${message.sender === 'user' ? 'justify-end' : 'justify-start'}`}>
                            <div
                                className={`max-w-[75%] p-3 rounded-lg ${
                                    message.sender === 'user'
                                        ? 'bg-[#8b8b7a] text-white'
                                        : 'bg-[#5f5f55] text-white'
                                }`}
                                style={{
                                    wordBreak: 'break-word',
                                    overflowWrap: 'break-word',
                                }}
                            >
                                <p>{message.content}</p>
                                <p className="text-xs opacity-75 mt-1">
                                    {new Date(message.timestamp).toLocaleTimeString()}
                                </p>
                            </div>
                        </div>
                    ))}
                </div>
            </ScrollArea>
            <form onSubmit={handleSendMessage} className="p-4 border-t border-[#55554d] bg-[#55554d] flex items-center space-x-2">
                <Input
                    value={newMessage}
                    onChange={(e) => setNewMessage(e.target.value)}
                    placeholder="Type a message..."
                    className="flex-grow bg-white text-[#44443c] placeholder-[#44443c]/50 border-[#66665b] focus:border-[#77776b] focus:ring focus:ring-[#77776b] focus:ring-opacity-50"
                />
                <Button
                    type="submit"
                    size="icon"
                    disabled={!wsReady}
                    className="bg-[#66665b] text-white hover:bg-[#77776b] focus:outline-none focus:ring-2 focus:ring-[#77776b] focus:ring-opacity-50"
                >
                    <Send className="h-4 w-4" />
                </Button>
            </form>
        </div>
    )
}