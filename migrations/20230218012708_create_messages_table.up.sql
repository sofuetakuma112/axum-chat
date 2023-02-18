-- Add up migration script here
CREATE TABLE IF NOT EXISTS messages (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id int NOT NULL references users(id),
    room_id int NOT NULL references rooms(id),
    message text NOT NULL,
    created_at timestamp NOT NULL default CURRENT_TIMESTAMP
)