#[cfg(test)]
mod test {

    use my_proc_macro::funtion_to_string;

    const OUTPUT: &str = "";

    #[funtion_to_string]
    fn some_function_for_ai_llm(_whatever_param: &str) {
        
    }

    #[test]
    fn test_proc_macros(){
        dbg!("Hello");
        let x = some_function_for_ai_llm("Wadidaw");
        dbg!(x);
    }
}