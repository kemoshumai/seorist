use web_sys::{AudioContext, GainNode};

pub struct Synth{
    ctx: AudioContext,
    output_gain: GainNode,
}

impl Synth{
    pub fn new() -> Self{
        let ctx = AudioContext::new().unwrap();

        // gainで音量を下げる
        let gain = ctx.create_gain().unwrap();
        gain.gain().set_value(0.025);
        gain.connect_with_audio_node(&ctx.destination()).unwrap();

        Self{
            ctx,
            output_gain: gain,
        }
    }

    pub fn play(&self, frequencies: &[f32]){
        
        for freq in frequencies {
            
            let osc = self.ctx.create_oscillator().unwrap();
            osc.set_type(web_sys::OscillatorType::Square);
            osc.connect_with_audio_node(self.output_gain.as_ref()).unwrap();

            osc.frequency().set_target_at_time(*freq, self.ctx.current_time(), 0.01).unwrap();
            osc.frequency().set_target_at_time(0., self.ctx.current_time()+1.0, 0.01).unwrap();
            osc.start().unwrap();

        }

    }
}