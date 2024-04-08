use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use self::utils::{Notifiable, ToastType};

pub mod component_factory;
pub mod component;
pub mod message;
pub mod manager;
pub mod provider;
pub mod use_toast;
pub mod utils;


#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub(crate) id: Uuid,
    pub(crate) toast_type: ToastType,
    pub(crate) title: String,
    pub(crate) text: String,
    pub(crate) classes: Option<String>,
    pub(crate) spawn_time: OffsetDateTime,
    pub(crate) lifetime: Duration,
    pub(crate) full_lifetime: Duration,
    pub(crate) paused: bool,
}

impl Toast {
    pub const LIFETIME: Duration = Duration::milliseconds(3000);

    pub fn new(
        toast_type: ToastType,
        title: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            toast_type,
            title: title.into(),
            text: text.into(),
            classes: Some("toast-loading".to_owned()),
            spawn_time: OffsetDateTime::now_local().expect("Unable to get current time"),
            lifetime: Self::LIFETIME,
            full_lifetime: Self::LIFETIME,
            paused: false,
        }
    }
}

impl Notifiable for Toast {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply_tick(&mut self, time: Duration) {
        self.lifetime = self.lifetime.checked_sub(time).unwrap_or_default();

        if self.lifetime < Self::LIFETIME - Duration::milliseconds(500) {
            self.classes = None;
        }
    }

    fn is_alive(&self) -> bool {
        self.lifetime != Duration::default()
    }

    fn mouse_in(&mut self) {
        self.paused = true;
    }

    fn mouse_out(&mut self) {
        self.paused = false;
        self.lifetime = self.full_lifetime;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}