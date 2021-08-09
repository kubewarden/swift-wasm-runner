use wasmtime::{AsContextMut, Func, FuncType, Val, ValType};

pub(crate) fn guest_request_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![ValType::I32, ValType::I32], vec![]);
    Func::new(
        store,
        callback_type,
        move |mut _caller, _params, _results| {
            println!("guest_request invoked");
            Ok(())
        },
    )
}

pub(crate) fn guest_response_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![ValType::I32, ValType::I32], vec![]);
    Func::new(
        store,
        callback_type,
        move |mut _caller, _params: &[Val], _results: &mut [Val]| {
            println!("guest_response invoked");
            Ok(())
        },
    )
}

pub(crate) fn guest_error_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![ValType::I32, ValType::I32], vec![]);
    Func::new(
        store,
        callback_type,
        move |mut _caller, _params: &[Val], _results: &mut [Val]| {
            println!("guest_error invoked");
            Ok(())
        },
    )
}

pub(crate) fn host_call_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(
        vec![
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
        ],
        vec![ValType::I32],
    );
    Func::new(
        store,
        callback_type,
        move |mut _caller, _params: &[Val], results: &mut [Val]| {
            println!("host_call invoked");
            results[0] = Val::I32(1);
            Ok(())
        },
    )
}

pub(crate) fn host_error_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![ValType::I32], vec![]);
    Func::new(
        store,
        callback_type,
        move |mut _caller, _params: &[Val], _results: &mut [Val]| {
            println!("host_error invoked");
            Ok(())
        },
    )
}

pub(crate) fn host_error_len_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![], vec![ValType::I32]);
    Func::new(
        store,
        callback_type,
        move |_caller, _params: &[Val], _results: &mut [Val]| {
            println!("host_error_len invoked");
            Ok(())
        },
    )
}

pub(crate) fn host_response_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![ValType::I32], vec![]);
    Func::new(
        store,
        callback_type,
        move |mut _caller, _params: &[Val], _results: &mut [Val]| {
            println!("host_response invoked");
            Ok(())
        },
    )
}

pub(crate) fn host_response_len_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![], vec![ValType::I32]);

    Func::new(
        store,
        callback_type,
        move |_caller, _params: &[Val], results: &mut [Val]| {
            println!("host_response_len invoked");
            results[0] = Val::I32(0);
            Ok(())
        },
    )
}

pub(crate) fn console_log_func(store: impl AsContextMut) -> Func {
    let callback_type = FuncType::new(vec![ValType::I32, ValType::I32], vec![]);

    Func::new(
        store,
        callback_type,
        move |mut _caller, _params: &[Val], _results: &mut [Val]| {
            println!("console_log invoked");
            Ok(())
        },
    )
}
