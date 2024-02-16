pub struct JavaScriptRuntime {
}

impl JavaScriptRuntime {
    pub fn new() -> JavaScriptRuntime {
        JavaScriptRuntime {}
    }

    pub fn execute(&mut self, _filename: &str, _source: &str) -> Result<String, String> {
        Ok("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let mut runtime = JavaScriptRuntime::new();
        {
            // a simple math
            let r = runtime.execute("", "1 + 1");
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "2");
        }
        {
            // simple string operation
            let r = runtime.execute("", "'test' + \"func\" + `012${1+1+1}`");
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "testfunc0123");
        }
        {
            // use of undefined variable
            let r = runtime.execute("", "test");
            assert!(r.is_err());
        }
        {
            // lambda definition
            let r = runtime.execute("", "let inc = (i) => { return i + 1 }; inc(1)");
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "2");
        }
        {
            // variable reuse
            let r = runtime.execute("", "inc(4)");
            assert!(r.is_ok());
            assert_eq!(r.unwrap(), "5");
        }
    }
}
