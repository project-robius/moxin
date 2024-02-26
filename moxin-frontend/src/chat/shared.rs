use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    ChatRole = <RoundedView> {
        width: 80,
        height: Fit,

        align: {x: 0.5, y: 0.5},
        padding: 16,

        draw_bg: {
            border_color: #000,
            border_width: 1.0,
        }

        label = <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 14},
                color: #000
            }
            text: "USER"
        }
    }
}