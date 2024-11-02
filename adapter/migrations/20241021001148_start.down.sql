-- Add down migration script here
DROP FUNCTION set_updated_at;

DROP TRIGGER IF EXISTS quiz_questions_updated_at_trigger ON quiz_questions;
DROP TRIGGER IF EXISTS quiz_answers_updated_at_trigger ON quiz_answers;
DROP TRIGGER IF EXISTS scored_quiz_updated_at_trigger ON scored_quiz;

DROP TABLE IF EXISTS quiz_questions;
DROP TABLE IF EXISTS quiz_answers;
DROP TABLE IF EXISTS cooked_quizzes;
DROP TABLE IF EXISTS recipe;
DROP TABLE IF EXISTS scored_quiz;