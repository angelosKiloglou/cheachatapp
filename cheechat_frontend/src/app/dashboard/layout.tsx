import NavigationBar from "@/components/navigation-bar";
import ChatList from "@/components/chat-list";

export default function DashboardLayout({
                                       children,
                                   }: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <div className="flex flex-col h-screen">
            {/* Fixed Navigation Bar */}
            <NavigationBar />
            <ChatList />
            {children}
        </div>
    );
}