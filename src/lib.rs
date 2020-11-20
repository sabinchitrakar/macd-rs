#![feature(external_doc)]

use ema_rs::EMA;
use ta_common::traits::Indicator;
#[doc(include = "../README.md")]
pub struct MACD {
    long_period: u32,

    short_ema: EMA,
    long_ema: EMA,
    signal_ema: EMA,
    current: u32,
}

impl MACD {
    pub fn new(short_period: u32, long_period: u32, signal_period: u32) -> MACD {
        Self {
            long_period,

            short_ema: EMA::new(short_period),
            long_ema: EMA::new(long_period),
            signal_ema: EMA::new(signal_period),
            current: 0,
        }
    }
}

impl Indicator<f64, Option<[f64; 3]>> for MACD {
    fn next(&mut self, input: f64) -> Option<[f64; 3]> {
        self.current = self.current + 1;
        let short = self.short_ema.next(input);
        let long = self.long_ema.next(input);
        return if self.current >= self.long_period {
            let macd = short - long;
            let signal = self.signal_ema.next(macd);
            let histogram = macd - signal;
            Some([macd, signal, histogram])
        } else {
            None
        };
    }

    fn reset(&mut self) {
        self.long_ema.reset();
        self.short_ema.reset();
        self.signal_ema.reset();
        self.current = 0;
    }
}


#[cfg(test)]
mod tests {
    use crate::MACD;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut macd = MACD::new(2, 5, 9);
        assert_eq!(macd.next(81.59), None);
        assert_eq!(macd.next(81.06), None);
        assert_eq!(macd.next(82.87), None);
        assert_eq!(macd.next(83.00), None);
        assert_eq!(macd.next(83.61), Some([0.6177777777777749, 0.6177777777777749, 0.00]));
        assert_eq!(macd.next(83.15), Some([0.3512757201646082, 0.5644773662551416, -0.21320164609053338]));
        assert_eq!(macd.next(82.84), Some([0.11065843621399551, 0.4737135802469124, -0.3630551440329169]));
        assert_eq!(macd.next(83.99), Some([0.41593049839961793, 0.46215696387745353, -0.0462264654778356]));
        assert_eq!(macd.next(84.55), Some([0.5780064014631705, 0.48532685139459697, 0.09267955006857354]));
        assert_eq!(macd.next(84.36), Some([0.4222440684854689, 0.4727102948127714, -0.050466226327302466]));
        assert_eq!(macd.next(85.53), Some([0.6837982014936586, 0.5149278761489489, 0.1688703253447097]));
        assert_eq!(macd.next(86.54), Some([0.9266328529413386, 0.5972688715074268, 0.32936398143391177]));
        assert_eq!(macd.next(86.89), Some([0.8913443637205063, 0.6560839699500427, 0.2352603937704636]));
        assert_eq!(macd.next(87.77), Some([0.9787592852891009, 0.7206190330178543, 0.2581402522712466]));
        assert_eq!(macd.next(87.29), Some([0.6206827600178713, 0.7006317784178577, -0.07994901839998647]));
    }
}
