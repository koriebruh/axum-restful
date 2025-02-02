CREATE DATABASE axum_rust;
USE axum_rust;

SHOW TABLES;

CREATE TABLE users
(
    id         int auto_increment primary key,
    username   varchar(255) unique,
    password   varchar(255),
    email      varchar(255) unique,
    created_at bigint,
    updated_at bigint
) ENGINE = InnoDB;

select *
from users;

select 1;