use crate::SideGainParams;
use nih_plug::params::Param;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

#[derive(Lens)]
struct Data {
    params: Arc<SideGainParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 350))
}

pub(crate) fn create(
    params: Arc<SideGainParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |ctx, _| {
        assets::register_noto_sans_regular(ctx);

        Data {
            params: params.clone(),
        }
        .build(ctx);

        VStack::new(ctx, |ctx| {
            Label::new(ctx, "SideGain")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Regular)
                .font_size(24.0)
                .height(Pixels(30.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            HStack::new(ctx, |ctx| {
                VStack::new(ctx, |ctx| {
                    Label::new(ctx, params.ratio_p_to_main.name());
                    ParamSlider::new(ctx, Data::params, |params| &params.ratio_p_to_main)
                        .width(Percentage(100.0));

                    Label::new(ctx, params.ratio_0_to_main.name());
                    ParamSlider::new(ctx, Data::params, |params| &params.ratio_0_to_main)
                        .width(Percentage(100.0));

                    Label::new(ctx, params.ratio_n_to_main.name());
                    ParamSlider::new(ctx, Data::params, |params| &params.ratio_n_to_main)
                        .width(Percentage(100.0));
                })
                .width(Percentage(33.0));
            })
            .col_between(Pixels(0.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Pixels(0.0))
        .child_right(Pixels(0.0));
    })
}
