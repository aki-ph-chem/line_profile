use gnuplot;

const PI:f64 = std::f64::consts::PI;
struct LorentzShape {
    width: f64,
}

impl LorentzShape {
    fn new(width: f64) -> LorentzShape {
        LorentzShape{width}
    }

    fn value(&self, x: f64, x_centor: f64) -> f64 {
        (self.width/2.0 * PI) / ((x - x_centor).powi(2) + (self.width / 2.0).powi(2)) 
    }
}

struct GaussianShape {
    width: f64,
}

impl GaussianShape {
    fn new(width: f64) -> GaussianShape {
        GaussianShape{width}
    }

    fn value(&self, x: f64, x_centor: f64) -> f64 {
        let coeff = ((2.0_f64).ln() / PI).sqrt() 
            / (self.width / 2.0);

        coeff * (-(x - x_centor).powi(2) / (self.width / 2.0)).exp()
    }
}

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

fn main() {
    // 線幅なしの信号
    let x_value = vec![0.5, 1.2, 2.3, 2.4, 3.5];
    let y_value = vec![1.0, 1.6, 3.0, 1.6, 1.2];

    // widthが0.004のLorentz関数
    let lorentz_profile = LorentzShape::new(0.004);

    let x_signal_lorentz:Vec<f64> = Range::new(0.1, 4.0, 0.01).collect();
    let mut y_signal_lorentz: Vec<f64> = vec![0.0; x_signal_lorentz.len()];

    for (i,_) in x_value.iter().enumerate() {
        for (j,_) in x_signal_lorentz.iter().enumerate() {
            y_signal_lorentz[j] 
                +=  y_value[i] * lorentz_profile.value(x_signal_lorentz[j], x_value[i]); 
        } 
    }

    // 1に規格化
    let mut max_sig_lorentz = -100.0;
    for sig in &y_signal_lorentz {
        if *sig > max_sig_lorentz {
            max_sig_lorentz = *sig;
        }
    }
    for sig in &mut y_signal_lorentz {
        *sig /= max_sig_lorentz;
    }

    // widthが0.004のGauss関数
    let gaussian_shape = GaussianShape::new(0.004);

    let x_signal_gauss:Vec<f64> = Range::new(0.1, 4.0, 0.01).collect();
    let mut y_signal_gauss: Vec<f64> = vec![0.0; x_signal_gauss.len()];

    for (i,_) in x_value.iter().enumerate() {
        for (j,_) in x_signal_gauss.iter().enumerate() {
            y_signal_gauss[j]
                += y_value[i] * gaussian_shape.value(x_signal_gauss[j], x_value[i]); 
        }
    }

    // 1に規格化
    let mut max_sig_gauss = -100.0;
    for sig in &y_signal_gauss {
        if *sig > max_sig_gauss {
            max_sig_gauss = *sig;
        }
    }
    for sig in &mut y_signal_gauss {
        *sig /= max_sig_gauss;
    }

    let mut fig = gnuplot::Figure::new();
    fig.axes2d()
        .lines(
            &x_signal_lorentz,
            &y_signal_lorentz,
            &[gnuplot::Caption("Lorentzian"), gnuplot::Color("#F15B5B")]
              )
        .lines(
            &x_signal_gauss,
            &y_signal_gauss,
            &[gnuplot::Caption("Gaussian"), gnuplot::Color("#67A8DD")]
              );

    fig.show().unwrap();
}
