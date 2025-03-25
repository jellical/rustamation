create table "user"
(
    user_id       uuid primary key not null,
    username      text unique      not null,
    password_hash text             not null
);