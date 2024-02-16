// V8についての解説
// Qiita: https://qiita.com/komukomo/items/316afadd04f95808f338

use std::{cell::RefCell, f32::consts::E, rc::Rc, sync::Once};

// use rusty_v8 as v8;
use v8;

use crate::dom::Node;

mod renderapi;

pub struct JavaScriptRuntimeState {
    pub context: v8::Global<v8::Context>,
    // pub renderer_api: Rc<RendererAPI>,
    // pub doccument_element: Rc<RefCell<Box<Node>>>,
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
            // let platform = v8::new_default_platform().unwrap();
            let platform = v8::new_default_platform(0, false).make_shared();
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

    pub fn execute(&mut self, filename: &str, source: &str) -> Result<String, String> {
        // Ok("".to_string())
        let scope = &mut self.get_handle_scope();

        // source
        let source = v8::String::new(scope, source).unwrap();
        let source_map = v8::undefined(scope);
        let name = v8::String::new(scope, filename).unwrap();

        // resource origin
        let origin = v8::ScriptOrigin::new(
            scope,
            name.into(),
            0,
            0,
            false,
            0,
            source_map.into(),
            false,
            false,
            false,
        );

        // tyr catch
        let mut tc_scope = v8::TryCatch::new(scope);
        
        // compile
        let script = match v8::Script::compile(&mut tc_scope, source, Some(&origin)) {
            Some(script) => script,
            None => {
                assert!(tc_scope.has_caught());
                return Err(to_pretty_string(tc_scope));
            }
        };

        // run
        match script.run(&mut tc_scope) {
            Some(result) => {
                let result_str = result
                    .to_string(&mut tc_scope)
                    .unwrap()
                    .to_rust_string_lossy(&mut tc_scope);
                return Ok(result_str.to_string());
            },
            None => {
                assert!(tc_scope.has_caught());
                return Err(to_pretty_string(tc_scope));
            }
        }
    }
}

/// `JavaScriptRuntimeState` から状態を取り戻すための実装群
impl JavaScriptRuntime {
    /// `state` returns the runtime state stored in the given isolate.
    pub fn state(isolate: &v8::Isolate) -> Rc<RefCell<JavaScriptRuntimeState>> {
        let s = isolate
            .get_slot::<Rc<RefCell<JavaScriptRuntimeState>>>()
            .unwrap();
        s.clone()
    }

    /// `get_state` returns the runtime state for the runtime.
    pub fn get_state(&self) -> Rc<RefCell<JavaScriptRuntimeState>> {
        Self::state(&self.v8_isolate)
    }

    /// `get_handle_scope` returns [a handle scope](https://v8docs.nodesource.com/node-0.8/d3/d95/classv8_1_1_handle_scope.html) for the runtime.
    pub fn get_handle_scope(&mut self) -> v8::HandleScope {
        let context = self.get_context();
        v8::HandleScope::with_context(&mut self.v8_isolate, context)
    }

    /// `get_context` returns [a handle scope](https://v8docs.nodesource.com/node-0.8/df/d69/classv8_1_1_context.html) for the runtime.
    pub fn get_context(&mut self) -> v8::Global<v8::Context> {
        let state = self.get_state();
        let state = state.borrow();
        state.context.clone()
    }
}

fn to_pretty_string(mut try_catch: v8::TryCatch<v8::HandleScope>) -> String {
    let exception_string = try_catch.exception().unwrap().to_string(&mut try_catch).unwrap().to_rust_string_lossy(&mut try_catch);
    let message = try_catch.message().unwrap();
    let filename = message.get_script_resource_name(&mut try_catch)
        .map_or_else(
            || "(unknown)".into(),
            |s| {
                s.to_string(&mut try_catch).unwrap().to_rust_string_lossy(&mut try_catch)
            },
        );
    let line_number = message.get_line_number(&mut try_catch).unwrap_or_default();
    format!("{}:{}: {}", filename, line_number, exception_string)
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
