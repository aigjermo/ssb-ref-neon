use neon::prelude::*;

mod ssb_ref;

/// Convenience method to parse an argument without throwing
fn try_cast_as<T: Value>(val: Option<Handle<JsValue>>) -> Option<Handle<T>> {
    match val {
        Some(val) if val.is_a::<T>() => Some(val.downcast::<T>().unwrap()),
        _ => None,
    }
}

fn is_blob(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    match try_cast_as::<JsString>(cx.argument_opt(0)) {
        Some(arg) => Ok(cx.boolean(ssb_ref::is_blob(&arg.value()))),
        _ => Ok(cx.boolean(false)),
    }
}

fn parse_link(mut cx: FunctionContext) -> JsResult<JsValue> {
    match try_cast_as::<JsString>(cx.argument_opt(0)) {
        Some(str) => match ssb_ref::parse_link(&str.value()) {
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
    cx.export_function("isBlob", is_blob)?;
    cx.export_function("isBlobId", is_blob)?;

    cx.export_function("extract", extract)?;
    cx.export_function("parseLink", parse_link)?;

    cx.export_function("normalizeChannel", normalize_channel)?;

    Ok(())
});
