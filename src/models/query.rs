enum Token {
    Select(String),
    Field(String,String),
    From(String),
    Join(String),
    Where(Box<Token>),
    Condition(String,String,String)
    
}