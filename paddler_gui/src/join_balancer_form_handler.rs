use std::net::SocketAddr;

use crate::join_balancer_form_data::JoinBalancerFormData;

#[derive(Debug, Clone)]
pub enum Message {
    SetAgentName(String),
    SetBalancerAddress(String),
    SetSlotsCount(String),
    ToggleGpuDevice { index: usize, is_selected: bool },
    Connect,
    Cancel,
}

pub enum Action {
    None,
    Cancel,
    ConnectAgent {
        agent_name: Option<String>,
        management_address: String,
        slots: i32,
        gpu_devices: Vec<usize>,
    },
}

impl JoinBalancerFormData {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SetAgentName(name) => {
                self.agent_name = name;

                Action::None
            }
            Message::SetBalancerAddress(address) => {
                self.balancer_address = address;
                self.balancer_address_error = None;

                Action::None
            }
            Message::SetSlotsCount(slots) => {
                if slots.is_empty() || slots.chars().all(|character| character.is_ascii_digit()) {
                    self.slots_count = slots;
                    self.slots_error = None;
                }

                Action::None
            }
            Message::ToggleGpuDevice { index, is_selected } => {
                if is_selected {
                    self.gpu_devices.push(index);
                    self.gpu_devices.sort_unstable();
                } else if let Some(position) = self
                    .gpu_devices
                    .iter()
                    .position(|device| *device == index)
                {
                    self.gpu_devices.remove(position);
                }
                self.gpu_devices_error = None;

                Action::None
            }
            Message::Connect => self.validate_and_connect(),
            Message::Cancel => Action::Cancel,
        }
    }

    fn validate_and_connect(&mut self) -> Action {
        self.balancer_address_error = None;
        self.slots_error = None;
        self.gpu_devices_error = None;

        if self.balancer_address.is_empty() {
            self.balancer_address_error = Some("Cluster address is required.".to_owned());
        } else if self.balancer_address.parse::<SocketAddr>().is_err() {
            self.balancer_address_error =
                Some("Invalid address, expected format: IP:port".to_owned());
        }

        let slots = if self.slots_count.is_empty() {
            self.slots_error = Some("Number of slots is required.".to_owned());
            None
        } else {
            match self.slots_count.parse::<i32>() {
                Ok(slots) if slots > 0 => Some(slots),
                Ok(non_positive_slots) => {
                    log::debug!("User entered non-positive slot count: {non_positive_slots}");
                    self.slots_error = Some(
                        "Invalid number of slots (the number should be greater than zero)."
                            .to_owned(),
                    );
                    None
                }
                Err(error) => {
                    let message = match error.kind() {
                        std::num::IntErrorKind::PosOverflow => "Number of slots is too large.",
                        unexpected_kind => {
                            log::error!("Unexpected slots parse error: {unexpected_kind:?}");
                            "Invalid number of slots."
                        }
                    };
                    self.slots_error = Some(message.to_owned());
                    None
                }
            }
        };

        if self.gpu_devices.is_empty() {
            self.gpu_devices_error = Some("Select at least one device.".to_owned());
        }

        if self.balancer_address_error.is_some()
            || self.slots_error.is_some()
            || self.gpu_devices_error.is_some()
        {
            return Action::None;
        }

        let Some(slots) = slots else {
            return Action::None;
        };

        let agent_name = if self.agent_name.is_empty() {
            None
        } else {
            Some(self.agent_name.clone())
        };

        Action::ConnectAgent {
            agent_name,
            management_address: self.balancer_address.clone(),
            slots,
            gpu_devices: self.gpu_devices.clone(),
        }
    }
}
