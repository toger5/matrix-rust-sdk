pub mod widget_client_driver;
pub mod widget_matrix_driver;

use widget_client_driver::WidgetClientDriver;

use self::widget_matrix_driver::ActualWidgetMatrixDriver;

use super::{handler::MessageHandlerDriver, messages::Outgoing};

pub struct Driver<CD: WidgetClientDriver> {
    pub matrix_driver: ActualWidgetMatrixDriver,
    pub client_driver: CD,
    pub send_to_widget: dyn Fn(dyn Outgoing)->()
}

impl<CD: WidgetClientDriver> MessageHandlerDriver for Driver<CD> {
    #[must_use]
    fn initialise<'life0, 'async_trait>(
        &'life0 mut self,
        req: CapabilitiesReq,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Capabilities>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    fn send<'life0, 'async_trait, T>(
        &'life0 mut self,
        req: T,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<T::Response>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        T: 'async_trait + Outgoing,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let message = serde_json::to_string(&req);
        self.send_to_widget(message);
    }
}
