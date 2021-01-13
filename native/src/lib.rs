use neon::prelude::*;

mod ssb_ref;
use ssb_ref::Ref;

/// Convenience method to parse an argument without throwing
fn try_cast_as<T: Value>(val: Option<Handle<JsValue>>) -> Option<Handle<T>> {
    match val {
        Some(val) if val.is_a::<T>() => Some(val.downcast::<T>().unwrap()),
        _ => None,
    }
}

fn is_of_type<'a>(t: &str, mut cx: FunctionContext<'a>) -> JsResult<'a, JsBoolean> {
    match try_cast_as::<JsString>(cx.argument_opt(0)) {
        Some(arg) => match Ref::from(&arg.value()) {
            Ok(parsed) if parsed.type_str() == t && !parsed.has_query() => Ok(cx.boolean(true)),
            _ => Ok(cx.boolean(false)),
        },
        _ => Ok(cx.boolean(false)),
    }
}

fn parse_link(mut cx: FunctionContext) -> JsResult<JsValue> {
    match try_cast_as::<JsString>(cx.argument_opt(0)) {
        Some(str) => match ssb_ref::parse_query(&str.value()) {
            Some((link, query)) => {
                let obj_result = JsObject::new(&mut cx);

                let link = cx.string(link);
                obj_result.set(&mut cx, "link", link)?;

                if let Some(query) = query {
                    let obj_query = JsObject::new(&mut cx);
                    for (k, v) in query.into_iter() {
                        let k = cx.string(k);
                        let v = cx.string(v);
                        obj_query.set(&mut cx, k, v)?;
                    }
                    obj_result.set(&mut cx, "query", obj_query)?;
                }

                Ok(obj_result.upcast())
            }
            None => Ok(cx.undefined().upcast()),
        },
        _ => Ok(cx.undefined().upcast()),
    }
}

fn extract(mut cx: FunctionContext) -> JsResult<JsValue> {
    match try_cast_as::<JsString>(cx.argument_opt(0)) {
        Some(arg) => match ssb_ref::extract_link(&arg.value()) {
            Some(str) => Ok(cx.string(str).upcast()),
            None => Ok(cx.boolean(false).upcast()),
        },
        _ => Ok(cx.boolean(false).upcast()),
    }
}

fn normalize_channel(mut cx: FunctionContext) -> JsResult<JsValue> {
    match try_cast_as::<JsString>(cx.argument_opt(0)) {
        Some(arg) => match ssb_ref::normalize_channel_name(&arg.value()) {
            Some(name) => Ok(cx.string(name).upcast()),
            None => Ok(cx.null().upcast()),
        },
        _ => Ok(cx.null().upcast()),
    }
}

register_module!(mut cx, {
    cx.export_function("isBlob", |cx| is_of_type("blob", cx))?;
    cx.export_function("isBlobId", |cx| is_of_type("blob", cx))?;
    cx.export_function("isFeed", |cx| is_of_type("feed", cx))?;
    cx.export_function("isFeedId", |cx| is_of_type("feed", cx))?;

    cx.export_function("extract", extract)?;
    cx.export_function("parseLink", parse_link)?;

    cx.export_function("normalizeChannel", normalize_channel)?;

    Ok(())
});
