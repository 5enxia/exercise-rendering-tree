// V8についての解説
// Qiita: https://qiita.com/komukomo/items/316afadd04f95808f338

use std::{cell::RefCell, rc::Rc, sync::Once};

use rusty_v8 as v8;

pub struct JavaScriptRuntimeState {
    pub context: v8::Global<v8::Context>
}

#[derive(Debug)]
pub struct JavaScriptRuntime {
    v8_isolate: v8::OwnedIsolate,
}

impl JavaScriptRuntime {
    pub fn new() -> JavaScriptRuntime {
        static PUPPY_INIT : Once = Once::new();
        PUPPY_INIT.call_once(move || {
            // Initialize V8.
            let platform = v8::new_default_platform().unwrap();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        // isolate
        // 隔離された実行環境
        // この中で実行されるコードは他のisolateに影響を与えない
        // 複数のisolateを作成することで、複数のスレッドで並列にJavaScriptを実行できる
        // メインインスタンスとWorkerインスタンスで別々のisolateを作成することで、並列実行が可能
        let mut isolate  = v8::Isolate::new(Default::default());
        
        // context
        // JavaScriptのSandBox化された実行環境
        // iframeがあるページにおいて、メインのwindowとは別にiframeのcontextが設けられる
        // また、Chrome拡張も別のcontextで実行される
        // 実行環境は異なるが、同じisolate内でこれらは実行されるため、並列実行はできない
        let context = {
            let isolate_scope = &mut v8::HandleScope::new(&mut isolate);
            let handle_scope = &mut v8::EscapableHandleScope::new(isolate_scope);
            let context = v8::Context::new(handle_scope);
            let context_scope = handle_scope.escape(context);
            v8::Global::new(handle_scope, context_scope)
        };

        isolate.set_slot(Rc::new(RefCell::new(JavaScriptRuntimeState {
            context 
        })));

        JavaScriptRuntime {
            v8_isolate: isolate,
        }
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
