use leptos::{html::Iframe, prelude::*};

pub struct SaveTimestamps {
    pub value: ReadSignal<Option<Box<[Option<String>; 15]>>>,
    pub setter: WriteSignal<Option<Box<[Option<String>; 15]>>>,
    refetch: RwSignal<bool>,
}

impl std::ops::Deref for SaveTimestamps {
    type Target = ReadSignal<Option<Box<[Option<String>; 15]>>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SaveTimestamps {
    pub fn new(frame: NodeRef<Iframe>) -> Self {
        let (value, setter) = signal(Default::default());
        let refetch = RwSignal::new(false);

        let mut previous = false;
        Effect::new(move || {
            let value = refetch();
            if value == previous {
                return;
            }
            previous = value;

            super::State::send_frame(frame, common::EngineMessage::GetSaveTimestamps);
        });

        Self {
            value,
            setter,
            refetch,
        }
    }

    pub fn refetch(&self) {
        self.refetch.update(|val| *val = !*val);
    }
}
