-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN
        new.updated_at := ''now'';
        return new;
    END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS quiz_questions(
    quiz_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question TEXT NOT NULL,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TRIGGER quiz_questions_updated_at_trigger
    BEFORE UPDATE ON quiz_questions FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();


CREATE TABLE IF NOT EXISTS quiz_answers(
    answer_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id UUID REFERENCES quiz_questions(quiz_id),
    answer TEXT NOT NULL,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TRIGGER quiz_answers_updated_at_trigger
    BEFORE UPDATE ON quiz_answers FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();

CREATE TABLE IF NOT EXISTS cooked_quizzes(
    cooked_quiz_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cooked_questions TEXT NOT NULL,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS recipe (
    recipe_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cooked_quiz_id UUID REFERENCES cooked_quizzes(cooked_quiz_id),
    quiz_id UUID REFERENCES quiz_questions(quiz_id),
    maked_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS scored_quiz(
    score_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cooked_quiz_id UUID REFERENCES cooked_quizzes(cooked_quiz_id),
    score INTEGER,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TRIGGER scored_quiz_updated_at_trigger
    BEFORE UPDATE ON scored_quiz FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();