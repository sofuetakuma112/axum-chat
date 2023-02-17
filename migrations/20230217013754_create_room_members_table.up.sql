-- Add up migration script here
CREATE TABLE IF NOT EXISTS room_members (
    room_id int references rooms(id),
    member_id int references users(id),
    PRIMARY KEY(room_id, member_id)
);