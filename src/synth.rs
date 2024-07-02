use web_sys::{AudioContext, GainNode, OscillatorNode};

pub struct Synth{
    ctx: AudioContext,
    output_gain: GainNode,
    oscillators: Vec<OscillatorNode>,
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
            oscillators: Vec::new(),
        }
    }

    pub fn play(&mut self, frequencies: &[f32]){
        
        for freq in frequencies {
            
            let osc = self.ctx.create_oscillator().unwrap();
            osc.set_type(web_sys::OscillatorType::Square);
            osc.connect_with_audio_node(self.output_gain.as_ref()).unwrap();

            osc.frequency().set_target_at_time(*freq, self.ctx.current_time(), 0.01).unwrap();
            osc.start().unwrap();

            self.oscillators.push(osc);

        }

    }

    pub fn stop(&mut self){
        for osc in self.oscillators.drain(..){
            osc.stop().unwrap();
        }
    }
}