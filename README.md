# Cheechat application
This is the fullstack repository of the application cheechat. It consists of two sub repositories, one for the client
and one for the server. The purpose of the project is completely for learning.

## Server
The server is implemented with the actix-web framework. Inspired from https://github.com/actix/examples, which provides
many basic components of a web applications with actix web. On this application, users can:
1. Register and login to the server. Redis session authentication is chosen.
2. Search for other users and initiate a private chat session with them.
3. Send messages in real time via a websocket. The message history is stored.
4. All the initiated chats are available to the user.

The server is completely non blocking as it is build on the actix async runtime. Additionally, the database operations 
when the users are chatting, are performed concurrently. Postgres is used for the database layer.

Note: To run this code, a postgres database and redis should be deployed and reference them to the .env file.

## Client 
The client is written in next.js. It consumes all the server api and exploit the shadcn/ui for elegant design.