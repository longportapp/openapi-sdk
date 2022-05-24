use longbridge::trade::{PushEvent, PushOrderChanged};
use pyo3::{PyErr, PyObject, Python};

pub(crate) fn handle_push_event(handler: &PyObject, event: PushEvent) {
    match event {
        PushEvent::OrderChanged(order_changed) => handle_order_changed(handler, order_changed),
    }
}

fn handle_order_changed(handler: &PyObject, order_changed: PushOrderChanged) {
    let _ = Python::with_gil(|py| {
        handler.call_method1(
            py,
            "on_event",
            (crate::trade::types::PushOrderChanged::try_from(
                order_changed,
            )?,),
        )?;
        Ok::<_, PyErr>(())
    });
}
