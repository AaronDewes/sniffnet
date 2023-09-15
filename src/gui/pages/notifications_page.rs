use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::tooltip::Position;
use iced::widget::{button, vertical_space};
use iced::widget::{lazy, Column, Container, Row, Scrollable, Text, Tooltip};
use iced::Length::FillPortion;
use iced::{Alignment, Font, Length, Renderer};

use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_emojis::FLAGS_WIDTH_BIG;
use crate::gui::components::header::get_button_settings;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_FOOTER};
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::notifications::types::logged_notification::{
    BytesThresholdExceeded, FavoriteTransmitted, LoggedNotification, PacketsThresholdExceeded,
};
use crate::translations::translations::{
    bytes_exceeded_translation, bytes_exceeded_value_translation, clear_all_translation,
    favorite_transmitted_translation, incoming_translation, no_notifications_received_translation,
    no_notifications_set_translation, only_last_30_translation, outgoing_translation,
    packets_exceeded_translation, packets_exceeded_value_translation, per_second_translation,
    threshold_translation,
};
use crate::utils::formatted_strings::get_formatted_bytes_string_with_b;
use crate::utils::types::icon::Icon;
use crate::{Language, RunningPage, Sniffer, StyleType};

/// Computes the body of gui notifications page
pub fn notifications_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let notifications = sniffer.notifications;
    let font = get_font(sniffer.style);
    let font_headers = get_font_headers(sniffer.style);

    let mut tab_and_body = Column::new()
        .align_items(Alignment::Center)
        .height(Length::Fill);

    let tabs = get_pages_tabs(
        RunningPage::Notifications,
        font,
        font_headers,
        sniffer.language,
        sniffer.unread_notifications,
    );

    tab_and_body = tab_and_body
        .push(tabs)
        .push(vertical_space(Length::Fixed(15.0)));

    if notifications.packets_notification.threshold.is_none()
        && notifications.bytes_notification.threshold.is_none()
        && !notifications.favorite_notification.notify_on_favorite
        && sniffer.runtime_data.logged_notifications.is_empty()
    {
        let body = body_no_notifications_set(font, sniffer.language);
        tab_and_body = tab_and_body.push(body);
    } else if sniffer.runtime_data.logged_notifications.is_empty() {
        let body = body_no_notifications_received(font, sniffer.language, &sniffer.waiting);
        tab_and_body = tab_and_body.push(body);
    } else {
        let logged_notifications = lazy(
            (
                sniffer.runtime_data.tot_emitted_notifications,
                sniffer.runtime_data.logged_notifications.len(),
                sniffer.language,
                sniffer.style,
            ),
            move |_| lazy_logged_notifications(sniffer),
        );
        let body_row = Row::new()
            .width(Length::Fill)
            .push(
                Container::new(if sniffer.runtime_data.logged_notifications.len() < 30 {
                    Text::new("")
                } else {
                    Text::new(only_last_30_translation(sniffer.language)).font(font)
                })
                .padding(10)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            )
            .push(
                Scrollable::new(logged_notifications)
                    .direction(Direction::Vertical(ScrollbarType::properties())),
            )
            .push(
                Container::new(get_button_clear_all(font, sniffer.language))
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            );
        tab_and_body = tab_and_body.push(body_row);
    }

    Container::new(Column::new().push(tab_and_body)).height(Length::Fill)
}

fn body_no_notifications_set(
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    Column::new()
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(vertical_space(FillPortion(1)))
        .push(
            no_notifications_set_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(get_button_settings(
            font,
            language,
            SettingsPage::Notifications,
        ))
        .push(vertical_space(FillPortion(2)))
}

fn body_no_notifications_received(
    font: Font,
    language: Language,
    waiting: &str,
) -> Column<'static, Message, Renderer<StyleType>> {
    Column::new()
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(vertical_space(FillPortion(1)))
        .push(
            no_notifications_received_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(vertical_space(FillPortion(2)))
}

fn packets_notification_log(
    logged_notification: PacketsThresholdExceeded,
    language: Language,
    font: Font,
) -> Container<'static, Message, Renderer<StyleType>> {
    let threshold_str = format!(
        "{}: {} {}",
        threshold_translation(language),
        logged_notification.threshold,
        per_second_translation(language)
    );
    let mut incoming_str = " - ".to_string();
    incoming_str.push_str(incoming_translation(language));
    incoming_str.push_str(": ");
    incoming_str.push_str(&logged_notification.incoming.to_string());
    let mut outgoing_str = " - ".to_string();
    outgoing_str.push_str(outgoing_translation(language));
    outgoing_str.push_str(": ");
    outgoing_str.push_str(&logged_notification.outgoing.to_string());
    let content = Row::new()
        .align_items(Alignment::Center)
        .height(Length::Fill)
        .spacing(30)
        .push(
            Tooltip::new(
                Icon::PacketsThreshold.to_text().size(80),
                packets_exceeded_translation(language),
                Position::FollowCursor,
            )
            .font(font)
            .style(ContainerType::Tooltip),
        )
        .push(
            Column::new()
                .spacing(7)
                .width(Length::Fixed(250.0))
                .push(
                    Row::new()
                        .spacing(5)
                        .push(Icon::Clock.to_text())
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(
                    Text::new(packets_exceeded_translation(language))
                        .style(TextType::Title)
                        .font(font),
                )
                .push(
                    Text::new(threshold_str)
                        .style(TextType::Subtitle)
                        .size(FONT_SIZE_FOOTER)
                        .font(font),
                ),
        )
        .push(
            Column::new()
                .spacing(7)
                .push(
                    Text::new(packets_exceeded_value_translation(
                        language,
                        logged_notification.incoming + logged_notification.outgoing,
                    ))
                    .font(font),
                )
                .push(Text::new(incoming_str).font(font))
                .push(Text::new(outgoing_str).font(font)),
        );
    Container::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(800.0))
        .padding(10)
        .style(ContainerType::BorderedRound)
}

fn bytes_notification_log(
    logged_notification: BytesThresholdExceeded,
    language: Language,
    font: Font,
) -> Container<'static, Message, Renderer<StyleType>> {
    let mut threshold_str = threshold_translation(language);
    threshold_str.push_str(": ");
    threshold_str.push_str(&get_formatted_bytes_string_with_b(
        (logged_notification.threshold).into(),
    ));

    threshold_str.push_str(&format!(" {}", per_second_translation(language)));
    let mut incoming_str = " - ".to_string();
    incoming_str.push_str(incoming_translation(language));
    incoming_str.push_str(": ");
    incoming_str.push_str(&get_formatted_bytes_string_with_b(u128::from(
        logged_notification.incoming,
    )));
    let mut outgoing_str = " - ".to_string();
    outgoing_str.push_str(outgoing_translation(language));
    outgoing_str.push_str(": ");
    outgoing_str.push_str(&get_formatted_bytes_string_with_b(u128::from(
        logged_notification.outgoing,
    )));
    let content = Row::new()
        .spacing(30)
        .align_items(Alignment::Center)
        .height(Length::Fill)
        .push(
            Tooltip::new(
                Icon::BytesThreshold.to_text().size(80),
                bytes_exceeded_translation(language),
                Position::FollowCursor,
            )
            .font(font)
            .style(ContainerType::Tooltip),
        )
        .push(
            Column::new()
                .spacing(7)
                .width(Length::Fixed(250.0))
                .push(
                    Row::new()
                        .spacing(5)
                        .push(Icon::Clock.to_text())
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(
                    Text::new(bytes_exceeded_translation(language))
                        .style(TextType::Title)
                        .font(font),
                )
                .push(
                    Text::new(threshold_str)
                        .size(FONT_SIZE_FOOTER)
                        .style(TextType::Subtitle)
                        .font(font),
                ),
        )
        .push(
            Column::new()
                .spacing(7)
                .push(
                    Text::new(bytes_exceeded_value_translation(
                        language,
                        &get_formatted_bytes_string_with_b(u128::from(
                            logged_notification.incoming + logged_notification.outgoing,
                        )),
                    ))
                    .font(font),
                )
                .push(Text::new(incoming_str).font(font))
                .push(Text::new(outgoing_str).font(font)),
        );
    Container::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(800.0))
        .padding(10)
        .style(ContainerType::BorderedRound)
}

fn favorite_notification_log(
    logged_notification: FavoriteTransmitted,
    language: Language,
    font: Font,
) -> Container<'static, Message, Renderer<StyleType>> {
    let domain = logged_notification.host.domain;
    let country = logged_notification.host.country;
    let asn = logged_notification.host.asn;

    let mut domain_asn_str = domain;
    if !asn.name.is_empty() {
        domain_asn_str.push_str(&format!(" - {}", asn.name));
    }

    let row_flag_details = Row::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(get_flag_tooltip(
            country,
            FLAGS_WIDTH_BIG,
            logged_notification.data_info_host.is_local,
            logged_notification.data_info_host.traffic_type,
            language,
            font,
        ))
        .push(Text::new(domain_asn_str).font(font));

    let content = Row::new()
        .spacing(30)
        .align_items(Alignment::Center)
        .height(Length::Fill)
        .push(
            Tooltip::new(
                Icon::Star.to_text().size(80),
                favorite_transmitted_translation(language),
                Position::FollowCursor,
            )
            .font(font)
            .style(ContainerType::Tooltip),
        )
        .push(
            Column::new()
                .width(Length::Fixed(250.0))
                .spacing(7)
                .push(
                    Row::new()
                        .spacing(5)
                        .push(Icon::Clock.to_text())
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(
                    Text::new(favorite_transmitted_translation(language))
                        .style(TextType::Title)
                        .font(font),
                ),
        )
        .push(
            Column::new()
                .spacing(7)
                .width(Length::Fill)
                .push(row_flag_details),
        );
    Container::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(800.0))
        .padding(10)
        .style(ContainerType::BorderedRound)
}

fn get_button_clear_all(
    font: Font,
    language: Language,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let content = button(
        Icon::Bin
            .to_text()
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(50.0))
    .width(Length::Fixed(75.0))
    .on_press(Message::ShowModal(MyModal::ClearAll));

    Tooltip::new(content, clear_all_translation(language), Position::Top)
        .gap(5)
        .font(font)
        .style(ContainerType::Tooltip)
}

fn lazy_logged_notifications(sniffer: &Sniffer) -> Column<'static, Message, Renderer<StyleType>> {
    let font = get_font(sniffer.style);
    let mut ret_val = Column::new()
        .width(Length::Fixed(830.0))
        .padding(5)
        .spacing(10)
        .align_items(Alignment::Center);

    for logged_notification in &sniffer.runtime_data.logged_notifications {
        ret_val = ret_val.push(match logged_notification {
            LoggedNotification::PacketsThresholdExceeded(packet_threshold_exceeded) => {
                packets_notification_log(packet_threshold_exceeded.clone(), sniffer.language, font)
            }
            LoggedNotification::BytesThresholdExceeded(byte_threshold_exceeded) => {
                bytes_notification_log(byte_threshold_exceeded.clone(), sniffer.language, font)
            }
            LoggedNotification::FavoriteTransmitted(favorite_transmitted) => {
                favorite_notification_log(favorite_transmitted.clone(), sniffer.language, font)
            }
        });
    }
    ret_val
}
