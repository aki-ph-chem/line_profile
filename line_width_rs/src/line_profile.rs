const PI:f64 = std::f64::consts::PI;

pub struct LineProfile {
    width: f64,
}

impl LineProfile {
    pub fn new(width: f64) -> LineProfile {
        LineProfile{width}
    }
} 

pub trait Lorentzian {
    fn lorentz(&self, x: f64, x_centor: f64) -> f64;
}

pub trait Gaussian {
    fn gauss(&self, x: f64, x_centor: f64) -> f64;
}

impl Lorentzian for LineProfile {
    fn lorentz(&self, x: f64, x_centor: f64) -> f64 {
        (self.width/2.0 * PI) / ((x - x_centor).powi(2) + (self.width / 2.0).powi(2)) 
    }
}

impl Gaussian for LineProfile {
    fn gauss(&self, x: f64, x_centor: f64) -> f64 {
        let coeff = ((2.0_f64).ln() / PI).sqrt() 
            / (self.width / 2.0);

        coeff * (-(x - x_centor).powi(2) / (self.width / 2.0)).exp()
    }
}

pub struct VoigtProfile {
    profile_lorentz: LineProfile,
    profile_gauss: LineProfile,
    eta: f64,
}

impl VoigtProfile {
    pub fn new(profile_lorentz: LineProfile, profile_gauss: LineProfile, eta: f64) -> VoigtProfile {
        VoigtProfile{profile_lorentz, profile_gauss, eta}
    }

    pub fn voigt(&self, x: f64, x_centor: f64) -> f64 {
        self.eta * self.profile_lorentz.lorentz(x, x_centor)
            + (1.0 - self.eta) * self.profile_gauss.gauss(x, x_centor)
    }
}
