use crate::{client::Client, error::ClientError, room::Room};
use matrix_sdk::widget::WidgetDriver;
use tracing::{error, warn};

#[uniffi::export(callback_interface)]
pub trait WidgetDriverToWidgetObserver: Sync + Send {
    fn did_receive_update(&self);
}
pub struct WidgetDriver {
    inner: matrix_sdk::widget::WidgetDriver,
    to_widget_delegate: Arc<RwLock<Option<Box<dyn ToWidgetDelegate>>>>,
}

impl WidgetDriver {
    pub fn observe_to_widget(
        &self,
        observer: Box<dyn WidgetDriverToWidgetObserver>,
    ) -> Arc<TaskHandle> {
        let (_, mut to_widget_stream) = self.inner.to_widget_stream();

        Arc::new(TaskHandle::new(RUNTIME.spawn(async move {
            loop {
                if let Some(new_to_widget) = to_widget_stream.next().await {
                    observer.did_receive_update(new_to_widget);
                }
            }
        })))
    }
    pub fn set_to_widget_delegate(&self, delegate: Option<Box<dyn ToWidgetDelegate>>) {
        *self.to_widget_delegate.write().unwrap() = delegate;
    }
}
#[uniffi::export(callback_interface)]
pub trait ToWidgetDelegate: Sync + Send {
    fn to_widget(&self, request: serde_json::Value);
}

#[uniffi::export(callback_interface)]
pub trait CapabilityDelegate: Sync + Send {
    fn did_receive_capability_request(&self);
}
