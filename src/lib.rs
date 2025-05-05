use nih_plug::buffer::{ChannelSamples, SamplesIter};
use nih_plug::prelude::*;
use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

pub struct SideGain {
    params: Arc<SideGainParams>,
}

#[derive(Params)]
struct SideGainParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "ratio_p_to_main"]
    pub ratio_p_to_main: FloatParam,
    #[id = "ratio_0_to_main"]
    pub ratio_0_to_main: FloatParam,
    #[id = "ratio_n_to_main"]
    pub ratio_n_to_main: FloatParam,

    #[id = "ratio_p_to_aux_a"]
    pub ratio_p_to_aux_a: FloatParam,
    #[id = "ratio_0_to_aux_a"]
    pub ratio_0_to_aux_a: FloatParam,
    #[id = "ratio_n_to_aux_a"]
    pub ratio_n_to_aux_a: FloatParam,

    #[id = "ratio_p_to_aux_b"]
    pub ratio_p_to_aux_b: FloatParam,
    #[id = "ratio_0_to_aux_b"]
    pub ratio_0_to_aux_b: FloatParam,
    #[id = "ratio_n_to_aux_b"]
    pub ratio_n_to_aux_b: FloatParam,
}

impl Default for SideGain {
    fn default() -> Self {
        Self {
            params: Arc::new(SideGainParams::default()),
        }
    }
}

impl Default for SideGainParams {
    fn default() -> Self {
        Self {
            ratio_p_to_main: FloatParam::new(
                "main x [+1]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            ratio_0_to_main: FloatParam::new(
                "main x [0]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            ratio_n_to_main: FloatParam::new(
                "main x [-1]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            ratio_p_to_aux_a: FloatParam::new(
                "aux_a x [+1]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            ratio_0_to_aux_a: FloatParam::new(
                "aux_a x [0]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            ratio_n_to_aux_a: FloatParam::new(
                "aux_a x [-1]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            ratio_p_to_aux_b: FloatParam::new(
                "aux_b x [+1]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            ratio_0_to_aux_b: FloatParam::new(
                "aux_b x [0]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            ratio_n_to_aux_b: FloatParam::new(
                "aux_b x [-1]",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
        }
    }
}

impl Plugin for SideGain {
    const NAME: &'static str = "SideGain";
    const VENDOR: &'static str = "Durun";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "negiigainuki@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[new_nonzero_u32(2); 3],

            names: PortNames {
                aux_inputs: &["Trigger", "A", "B"],
                ..PortNames::const_default()
            },
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),

            aux_input_ports: &[new_nonzero_u32(1); 3],

            names: PortNames {
                aux_inputs: &["Trigger", "A", "B"],
                ..PortNames::const_default()
            },
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        main: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let trigger_input = &aux.inputs[0];

        // main input を計算する
        for (main_samples, trigger_samples) in main
            .as_slice()
            .iter_mut()
            .zip(trigger_input.as_slice_immutable())
        {
            for (main_sample, triger_sample) in main_samples.iter_mut().zip(trigger_samples.iter())
            {
                *main_sample *= calc_ratio(
                    *triger_sample,
                    self.params.ratio_p_to_main.value(),
                    self.params.ratio_0_to_main.value(),
                    self.params.ratio_n_to_main.value(),
                );
            }
        }

        // aux を足しこむ
        for (aux, mix_p, mix_0, mix_n) in [
            (
                &aux.inputs[1],
                &self.params.ratio_p_to_aux_a,
                &self.params.ratio_0_to_aux_a,
                &self.params.ratio_n_to_aux_a,
            ),
            (
                &aux.inputs[2],
                &self.params.ratio_p_to_aux_b,
                &self.params.ratio_0_to_aux_b,
                &self.params.ratio_n_to_aux_b,
            ),
        ] {
            for ((main_samples, trigger_samples), aux_samples) in main
                .as_slice()
                .iter_mut()
                .zip(trigger_input.as_slice_immutable())
                .zip(aux.as_slice_immutable())
            {
                for ((main_sample, triger_sample), aux_sample) in main_samples
                    .iter_mut()
                    .zip(trigger_samples.iter())
                    .zip(aux_samples.iter())
                {
                    *main_sample += aux_sample
                        * calc_ratio(*triger_sample, mix_p.value(), mix_0.value(), mix_n.value());
                }
            }
        }

        ProcessStatus::Normal
    }
}

fn calc_ratio(trigger: f32, param_p_mix: f32, param_0_mix: f32, param_n_mix: f32) -> f32 {
    if trigger < 0.0 {
        param_n_mix * (-trigger) + param_0_mix * (trigger + 1.0)
    } else {
        param_p_mix * trigger + param_0_mix * (-trigger + 1.0)
    }
}

impl ClapPlugin for SideGain {
    const CLAP_ID: &'static str = "com.github.durun.sidegain";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Audio Plugin that controlls gain by sidechain sample");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for SideGain {
    const VST3_CLASS_ID: [u8; 16] = *b"durun/sidegain__";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(SideGain);
nih_export_vst3!(SideGain);
