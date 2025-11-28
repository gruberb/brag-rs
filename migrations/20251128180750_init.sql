-- Add migration script here
CREATE TABLE IF NOT EXISTS entries
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    link        TEXT             ,
    date        INTEGER                 NOT NULL
);

CREATE TABLE IF NOT EXISTS collaborators(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    team TEXT NOT NULL,
    UNIQUE(name, team)

);