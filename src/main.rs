fn main() {
    let out = nereus::transform(
        "class MyModel(pydantic.BaseModel):\n\n    id: int\n    value: t.Any\n\n    def say_hello():\n        print('hello!')".to_string(),
    );
    dbg!("{}", out);
}
