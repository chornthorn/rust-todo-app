-- Add migration script here
DROP TABLE IF EXISTS todos;

CREATE TABLE todos
(
    id         INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title      VARCHAR(255) NOT NULL,
    completed  tinyint(1)   NOT NULL DEFAULT '0',
    created_at TIMESTAMP             DEFAULT CURRENT_TIMESTAMP
);