# Roadmap

## Upcoming topics
### Complete the chat functionality
1. Add chat history:
   1. Update the database schema by adding the chats and chat_messages tables
   2. Update the code: Create records to the tables when users send a message or creates a chat.
   3. Get all the recent messages when user opens an existing chat
2. Introduce group chats (Perform analysis)
3. Simple front end: Implement a minimal but enjoyable font end (check react??)
4. Testing

Note: Decouple the storing - db call from the message handling (send, broadcast message, handle websocket) for enhanced performance.

### Secure and structure/behaviour improvements- application layer
1. Make the application https and adapt the client (front end)
2. Introduce AppState to include all the application related data
3. Graceful shutdown of the server - check examples

## Following topics - abstracted
## End to end message encryption
Analyse the challenges and the methods for end to end message encryption. Try to get a deep insight to implement the end to end encryption layer.

## Add extra features
Some idea to extend the purpose of the application:
1. Add sharing folders and allow users to communicate via a context (same shared folders?)
2. Implement the initial idea without the challenge of embedded vector db (qsrant as candidate??) to perform semantic search via the common folder within the users.
3. Add common editing
4. Add administration rights and options for public folders etc




