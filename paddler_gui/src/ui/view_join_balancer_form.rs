use iced::Center;
use iced::Element;
use iced::Fill;
use iced::alignment::Horizontal;
use iced::widget::button;
use iced::widget::checkbox;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;

use super::font::BOLD;
use super::font::REGULAR;
use super::style_button_primary::style_button_primary;
use super::style_field_checkbox::style_field_checkbox;
use super::style_field_container::style_field_container;
use super::style_field_text_input::style_field_text_input;
use super::variables::COLOR_ERROR;
use super::variables::FONT_SIZE_BASE;
use super::variables::FONT_SIZE_L2;
use super::variables::FORM_WIDTH;
use super::variables::SPACING_2X;
use super::variables::SPACING_BASE;
use super::variables::SPACING_HALF;
use super::view_form_field::view_form_field;
use crate::gpu_devices::GpuDevice;
use crate::join_balancer_form_data::JoinBalancerFormData;
use crate::join_balancer_form_handler::Message;

pub fn view_join_balancer_form<'a>(
    data: &'a JoinBalancerFormData,
    devices: &'a [GpuDevice],
) -> Element<'a, Message> {
    let confirm_button = button(text("Connect").font(BOLD))
        .padding([SPACING_HALF, SPACING_BASE])
        .style(style_button_primary)
        .on_press(Message::Connect);

    let cancel_button = button(text("Cancel").font(BOLD))
        .style(button::text)
        .on_press(Message::Cancel);

    let balancer_address_input = text_input("IP:port", &data.balancer_address)
        .on_input(Message::SetBalancerAddress)
        .padding(SPACING_BASE)
        .style(style_field_text_input)
        .into();

    let agent_name_input = text_input("my-agent", &data.agent_name)
        .on_input(Message::SetAgentName)
        .padding(SPACING_BASE)
        .style(style_field_text_input)
        .into();

    let slots_input = text_input("e.g. 1", &data.slots_count)
        .on_input(Message::SetSlotsCount)
        .padding(SPACING_BASE)
        .style(style_field_text_input)
        .into();

    let devices_field = view_gpu_devices_field(data, devices);

    column![
        container(text("Join a cluster").size(FONT_SIZE_L2).font(BOLD))
            .padding([0.0, SPACING_BASE]),
        container(
            column![
                view_form_field(
                    "Cluster address",
                    balancer_address_input,
                    data.balancer_address_error.as_ref()
                ),
                view_form_field("Agent name (optional)", agent_name_input, None),
                view_form_field("Slots", slots_input, data.slots_error.as_ref()),
                devices_field,
                container(
                    row![cancel_button, confirm_button]
                        .align_y(Center)
                        .spacing(SPACING_BASE),
                )
                .align_x(Horizontal::Right),
            ]
            .spacing(SPACING_2X),
        )
        .width(FORM_WIDTH),
    ]
    .spacing(SPACING_2X)
    .into()
}

/// Maximum height of the device list before it scrolls internally
const DEVICE_LIST_MAX_HEIGHT: f32 = 180.0;

/// Height of a single line of checkbox text
const DEVICE_TEXT_LINE_HEIGHT: f32 = 20.0;

/// Spacing between device checkbox rows
const DEVICE_ROW_SPACING: f32 = 4.0;

/// Number of characters that fit on one line of the device label
const DEVICE_LABEL_CHARS_PER_LINE: usize = 55;

fn view_gpu_devices_field<'a>(
    data: &'a JoinBalancerFormData,
    devices: &'a [GpuDevice],
) -> Element<'a, Message> {
    let header = row![
        container(text("Inference devices").font(BOLD)).width(Fill),
        text("deselect to restrict").font(REGULAR),
    ];

    let mut checkboxes = column![].spacing(DEVICE_ROW_SPACING);

    // Total text lines across all devices, so the list can be sized to its content
    let mut total_text_lines = 0;

    for device in devices {
        let is_selected = data.gpu_devices.contains(&device.index);

        let lines =
            device.description.len().div_ceil(DEVICE_LABEL_CHARS_PER_LINE);
        total_text_lines += lines;

        checkboxes = checkboxes.push(
            container(
                checkbox(is_selected)
                    .label(device.description.clone())
                    .font(REGULAR)
                    .size(FONT_SIZE_BASE)
                    .text_size(FONT_SIZE_BASE)
                    .on_toggle(move |is_selected| Message::ToggleGpuDevice {
                        index: device.index,
                        is_selected,
                    })
                    .style(style_field_checkbox),
            )
            .padding([0.0, SPACING_BASE]),
        );
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "line and device counts are tiny and fit in f32 mantissa"
    )]
    let list_height = (devices.len() as f32 - 1.0)
        .mul_add(DEVICE_ROW_SPACING, total_text_lines as f32 * DEVICE_TEXT_LINE_HEIGHT)
        .min(DEVICE_LIST_MAX_HEIGHT);

    let device_list = container(
        scrollable(checkboxes)
            .height(list_height)
            .width(Fill),
    )
    .style(style_field_container)
    .padding(SPACING_HALF);

    let mut field = column![header, device_list].spacing(SPACING_BASE);

    if let Some(error) = data.gpu_devices_error.as_ref() {
        field = field.push(
            container(text(error.clone()).font(REGULAR).color(COLOR_ERROR))
                .padding([0.0, SPACING_BASE]),
        );
    }

    field.into()
}
