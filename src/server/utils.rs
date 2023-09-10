macro_rules! iface_delegate {
    ($iface:ty, $name:ident) => {
        pub async fn $name(&self) -> Result<()> {
            let iface_ref = self.interface_ref::<$iface>().await?;
            let iface = iface_ref.get().await;
            iface.$name(iface_ref.signal_context()).await
        }
    };
}

pub(crate) use iface_delegate;
