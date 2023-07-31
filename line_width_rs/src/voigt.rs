mod line_profile;
use line_profile::Lorentzian;
use line_profile::Gaussian;
use gnuplot;
use std::error;

struct Range {
    x_current: f64,
    x_fin: f64,
    step: f64,
}

impl Range {
    fn new(x_ini: f64, x_fin: f64, step: f64) -> Range {
        Range{x_current: x_ini, x_fin, step}
    }
}

impl Iterator for Range {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_current <= self.x_fin {
            let value = self.x_current;
            self.x_current += self.step;
            Some(value)
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let x_value = vec![0.5, 1.2, 2.3, 2.4, 3.5];
    let y_value = vec![1.0, 1.6, 3.0, 1.6, 1.2];
    let x_signal:Vec<f64> = Range::new(0.1, 4.0, 0.01).collect(); 

    let profile_1 = line_profile::LineProfile::new(0.004);
    let mut y_signal_lorentz:Vec<f64> = vec![0.0; x_signal.len()];
    for (i,_) in x_value.iter().enumerate() {
        for (j,_) in x_signal.iter().enumerate() {
            y_signal_lorentz[j] 
                +=  y_value[i] * profile_1.lorentz(x_signal[j], x_value[i]); 
        } 
    }

    let profile_2 = line_profile::LineProfile::new(0.004);
    let mut y_signal_gauss:Vec<f64> = vec![0.0; x_signal.len()];
    for (i,_) in x_value.iter().enumerate() {
        for (j,_) in x_signal.iter().enumerate() {
            y_signal_gauss[j] 
                +=  y_value[i] * profile_2.gauss(x_signal[j], x_value[i]); 
        } 
    }

    let profile_3 = line_profile::VoigtProfile::new(profile_1, profile_2, 0.5);
    let mut y_signal_voigt:Vec<f64> = vec![0.0; x_signal.len()];
    for (i,_) in x_value.iter().enumerate() {
        for (j,_) in x_signal.iter().enumerate() {
            y_signal_voigt[j] 
                +=  y_value[i] * profile_3.voigt(x_signal[j], x_value[i]); 
        } 
    }

    let mut fig = gnuplot::Figure::new();
    fig.axes2d()
        .lines(
            &x_signal,
            &y_signal_lorentz,
            &[gnuplot::Caption("Lorentzian"), gnuplot::Color("#F15B5B")]
              )
        .lines(
            &x_signal,
            &y_signal_gauss,
            &[gnuplot::Caption("Gaussian"), gnuplot::Color("#235BC8")]
              )
        .lines(
            &x_signal,
            &y_signal_voigt,
            &[gnuplot::Caption("Voigt"), gnuplot::Color("#5AFF19")]
              );
    fig.show()?;

    Ok(())
}
