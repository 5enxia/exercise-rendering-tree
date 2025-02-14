use v8::PropertyAttribute;

use crate::{
    dom::{Node, NodeType},
    javascript::{JavaScriptRuntime, JavaScriptRuntimeState},
};

use std::ffi::c_void;

// use v8::READ_ONLY;

type NodeRefTarget<'a> = &'a mut Box<Node>;

// v8にrustのNodeを渡す
fn to_v8_node<'s>(
    scope: &mut v8::HandleScope<'s>,
    node_rust: NodeRefTarget,
) -> v8::Local<'s, v8::Object> {
    // v8上にオブジェクトのテンプレートを作成
    let template = v8::ObjectTemplate::new(scope);
    template.set_internal_field_count(1);
    let node_v8 = template.new_instance(scope).unwrap();

    // rustのNodeをv8のオブジェクトに紐付ける
    let boxed_ref = Box::new(node_rust);
    let address = Box::leak(boxed_ref) as *mut NodeRefTarget as *mut c_void;
    let v8_external = v8::External::new(scope, address);
    let target_node_ref_v8: v8::Local<v8::Data> = v8_external.into();
    node_v8.set_internal_field(0, target_node_ref_v8);

    node_v8
}

// v8からNodeの情報を取得し、rustのNodeに紐づける
fn to_linked_rust_node<'s>(
    scope: &mut v8::HandleScope<'s>,
    node_v8: v8::Local<v8::Object>,
) -> &'s mut NodeRefTarget<'s> {
    let node_v8 = node_v8.get_internal_field(scope, 0).unwrap();
    let node = unsafe { v8::Local::<v8::External>::cast(node_v8) };
    let node = node.value() as *mut NodeRefTarget;
    unsafe { &mut *node }
}

// RustのNodeのうち、node_typeがElementのものをv8に渡す
fn to_v8_element<'s>(
    scope: &mut v8::HandleScope<'s>,
    tag_name: &str,
    _attributes: Vec<(String, String)>,
    node_rust: NodeRefTarget,
) -> v8::Local<'s, v8::Object> {
    // v8上にNodeオブジェクトを紐づける
    let node = to_v8_node(scope, node_rust);

    // tagNameプロパティをv8上に追加
    {
        let key = v8::String::new(scope, "tagName").unwrap();
        let value = v8::String::new(scope, tag_name).unwrap();
        node.define_own_property(
            scope,
            key.into(),
            value.into(),
            PropertyAttribute::READ_ONLY,
        );
    }

    // innnerHtmlプロパティをv8上に追加
    {
        let key = v8::String::new(scope, "innerHTML").unwrap();
        node.set_accessor_with_setter( 
            scope,
            key.into(),
            move |scope: &mut v8::HandleScope,
                  _key: v8::Local<v8::Name>,
                  args: v8::PropertyCallbackArguments,
                  mut rv: v8::ReturnValue| {
                let this = args.this();
                let node = to_linked_rust_node(scope, this);

                let ret = v8::String::new(scope, node.inner_html().as_str()).unwrap();
                rv.set(ret.into());
            },
            move |scope: &mut v8::HandleScope,
                  _key: v8::Local<v8::Name>,
                  value: v8::Local<v8::Value>,
                  args: v8::PropertyCallbackArguments,
                  mut _rv: v8::ReturnValue
                | {
                let this = args.this();
                let node = to_linked_rust_node(scope, this);
                node.set_inner_html(value.to_rust_string_lossy(scope).as_str());

                JavaScriptRuntime::renderer_api(scope).rerender();
            },
        );
    }

    node
}

// DOMを構築する
pub fn create_document_object<'s>(
    scope:&mut v8::ContextScope<'s, v8::EscapableHandleScope>,
) -> v8::Local<'s, v8::Object> {
    let document = v8::ObjectTemplate::new(scope).new_instance(scope).unwrap();

    {
        // getelementById()の関数定義
        let key = v8::String::new(scope, "getElementById").unwrap();
        let function_template = v8::FunctionTemplate::new(
            scope,
            |scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut return_val: v8::ReturnValue| {
                let id = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);

                let document_element = JavaScriptRuntime::document_element(scope);
                let document_element = &mut document_element.borrow_mut();

                return_val.set(
                    document_element
                        .get_element_by_id(id.as_str())
                        .and_then(|node| {
                            if let NodeType::Element(ref mut el) = node.node_type {
                                let tag_name = el.tag_name.clone();
                                let attributes = el.attributes();
                                Some((node, tag_name, attributes))
                            } else {
                                None
                            }
                        })
                        .and_then(|(node, tag_name, attributes)| {
                            Some(to_v8_element(scope, tag_name.as_str(), attributes, node).into())
                        })
                        .unwrap_or_else(|| v8::undefined(scope).into()),
                );
            },
        );

        let val = function_template.get_function(scope).unwrap();
        document.set(scope, key.into(), val.into());
    }

    document
}