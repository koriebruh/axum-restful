CREATE DATABASE axum_rust;
USE axum_rust;

SHOW TABLES;

CREATE TABLE users
(
    id         int auto_increment primary key,
    username   varchar(255) unique not null,
    password   varchar(255)        not null,
    email      varchar(255) unique not null,
    created_at bigint              not null,
    updated_at bigint
) ENGINE = InnoDB;

select *
from users;

select 1;

SELECT EXISTS (
    SELECT 1
    FROM users
    WHERE username = 'example_username'
);
