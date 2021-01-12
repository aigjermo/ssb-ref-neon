use neon::prelude::*;

mod ssb_ref;

fn is_blob(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let str = cx.argument::<JsString>(0)?.value();

    Ok(cx.boolean(ssb_ref::is_blob(&str)))
}

fn parse_link(mut cx: FunctionContext) -> JsResult<JsValue> {
    let str = cx.argument::<JsString>(0)?.value();

    match ssb_ref::parse_link(&str) {
        None => Ok(cx.undefined().upcast()),
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
    }
}

fn extract(mut cx: FunctionContext) -> JsResult<JsValue> {
    match cx.argument::<JsString>(0) {
        Err(_) => Ok(cx.boolean(false).upcast()),
        Ok(str) => match ssb_ref::extract_link(&(str.value())) {
            Some(str) => Ok(cx.string(str).upcast()),
            None => Ok(cx.boolean(false).upcast()),
        },
    }
}

register_module!(mut cx, {
    cx.export_function("isBlob", is_blob)?;
    cx.export_function("isBlobId", is_blob)?;

    cx.export_function("parseLink", parse_link)?;

    cx.export_function("extract", extract)?;

    Ok(())
});
