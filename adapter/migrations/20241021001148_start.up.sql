-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN
        new.updated_at := ''now'';
        return new;
    END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS quizzes(
    quiz_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS answers(
    answer_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id UUID REFERENCES quizzes(quiz_id),
    answer TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS good_quiz(
    quiz_id UUID REFERENCES quizzes(quiz_id),
    score INTEGER
);