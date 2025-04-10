fn main() {
    let lim = Body {
        height: 160.0,
        weidht: 70.0,
    };

    println!("BMI: {:.2}", lim.calc_bmi());
    println!("비만율: {:.1}%", lim.calc_percentile());
}

struct Body {
    height: f64,
    weidht: f64,
}

impl Body {
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        let bmi = self.weidht / h.powf(2.0);

        bmi
    }

    fn calc_percentile(&self) -> f64 {
        let bmi: f64 = self.calc_bmi();
        let percentile = bmi / 22.0 * 100.0;

        percentile
    }
}
