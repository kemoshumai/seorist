use web_sys::AudioContext;

pub struct Synth{
    ctx: AudioContext
}

impl Synth{
    pub fn new() -> Self{
        let ctx = AudioContext::new().unwrap();
        Self{
            ctx
        }
    }

    pub fn play(&self, freq: f32){
        let osc = self.ctx.create_oscillator().unwrap();
        osc.frequency().set_value(freq);
        osc.connect_with_audio_node(&self.ctx.destination()).unwrap();
        osc.start().unwrap();
    }
}