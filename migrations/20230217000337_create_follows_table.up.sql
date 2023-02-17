-- Add up migration script here
CREATE TABLE IF NOT EXISTS follows (
    follower_id int references users(id),
    followee_id int references users(id),
    PRIMARY KEY(follower_id, followee_id)
);