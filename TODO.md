# Plan/Todo

In this quizzical crate, i want the following things:

- Serializeable/Deserializable Quizes/Questions to and from JSON.
- Different Question Types, using an api that lets people make custom questions,
  the builtin ones are, (Typed answer question, multiple choice, timed question)
  but we will make those once we have the custom question api made.
- Timed Quiz, (need to finish the quiz in a certain time)
- Custom Quiz types (Regular, Timed, Flashcards)
- Custom Scoring Mechanisms (Builtin one is percentage based, gives a percentage
  score)

Now we just need to focus on one thing at a time, we have a question.rs and
quiz.rs file. Make use of the following crates:

- rand: for shuffling the multiple choice, and maybe the question order
- serde: making the questions/quizes serializable/deserializable
- simd-json: making questions/quizes serializable/deserializable to/from json.
- thiserror: custom error types for error handling
- log: gate this behind a feature called "logging" and its added as a
  dev-dependency, it allows custom logging.
