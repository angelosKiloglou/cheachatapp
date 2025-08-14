# Cheechat 💬

Cheechat is a **full-stack chat application** built purely for learning purposes.  
It’s split into two sub-projects:  
- **Server** → Implemented in **Rust** with [Actix-Web](https://actix.rs/).  
- **Client** → Built with **Next.js** and styled using [shadcn/ui](https://ui.shadcn.com/).

---

## Features

### 🔐 Authentication
- **Register/Login** with **Redis session authentication**.
- Secure session management with server-side validation.

### 🗨 Chat Functionality
- Search for other users and start **private chat sessions**.
- Real-time messaging using **WebSockets**.
- All chats and their histories are persisted in **PostgreSQL** and available anytime.

### ⚡ Performance
- Fully **asynchronous & non-blocking** server runtime.
- Concurrent database operations during active chats.

---

## Technical Architecture

Cheechat’s server leverages the **actor model** for efficient concurrency and scalability:

- **Actors**
  - Each actor has its **own execution context** and communicates with others in a **thread-safe** manner.
- **Main Chat Server Actor**
  - Tracks active chat sessions.
  - Routes messages between connected clients.
- **Chat Session Actors**
  - Each session actor is tied to a **WebSocket connection** opened by a user.
  - Lives only while the WebSocket is active, when the connection closes, the actor is destroyed.

---

## Prerequisites
Before running the server, ensure you have:
- PostgreSQL instance
- Redis instance
- A `.env` file with correct database and Redis connection strings

---

## Inspiration
The server design was inspired by the [Actix examples](https://github.com/actix/examples),  
which provide practical building blocks for web applications in Rust.
