DROP SCHEMA IF EXISTS cheechat CASCADE;
CREATE SCHEMA cheechat;

CREATE TABLE cheechat.users (
	id  BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
    password    VARCHAR(255) UNIQUE NOT NULL,
	UNIQUE (username)
);

CREATE TABLE cheechat.chats (
                               id BIGSERIAL PRIMARY KEY,
                               user1_id BIGINT NOT NULL,
                               user2_id BIGINT NOT NULL,
                               created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                               last_message_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                               FOREIGN KEY (user1_id) REFERENCES cheechat.users(id),
                               FOREIGN KEY (user2_id) REFERENCES cheechat.users(id),
                               CONSTRAINT unique_chat UNIQUE (user1_id, user2_id)
);

CREATE TABLE cheechat.chat_messages (
                          id BIGSERIAL PRIMARY KEY,
                          chat_id BIGINT NOT NULL,
                          sender_id BIGINT NOT NULL,
                          message TEXT NOT NULL,
                          created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          FOREIGN KEY (chat_id) REFERENCES cheechat.chats(id),
                          FOREIGN KEY (sender_id) REFERENCES cheechat.users(id)
);

CREATE INDEX idx_chats_user1_id ON cheechat.chats(user1_id);
CREATE INDEX idx_chats_user2_id ON cheechat.chats(user2_id);

CREATE INDEX idx_chat_messages_chat_id_created_at ON cheechat.chat_messages(chat_id, created_at DESC);
