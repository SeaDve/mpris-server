macro_rules! changed_delegate {
    ($iface:ty, $name:ident) => {
        pub async fn $name(&self) -> Result<()> {
            let iface_ref = self.interface_ref::<$iface>().await?;
            let iface = iface_ref.get().await;
            iface.$name(iface_ref.signal_context()).await
        }
    };
}

macro_rules! signal_delegate {
    ($iface:ty, $name:ident($($arg_name:ident: $arg_ty:ty),*)) => {
        pub async fn $name(&self, $($arg_name: $arg_ty),*) -> Result<()> {
            let iface_ref = self.interface_ref::<$iface>().await?;
            <$iface>::$name(iface_ref.signal_context(), $($arg_name),*).await
        }
    };
}

pub(super) use {changed_delegate, signal_delegate};
