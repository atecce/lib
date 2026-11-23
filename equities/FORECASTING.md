# Outliers, forecasting, and efficient markets

Notes from 2026-11-22, after running augurs (Grafana's Rust time series
library) over the S&P 500's reported items: `examples/quality.rs` on the
`augurs` branch. See also [VALUATION.md](VALUATION.md).

## What the scan found

Within each series (a company's item over time), the checks found reader
errors, because errors have signatures that genuine business doesn't:

- **Exact zeros** where a company never reports zero: 823 series, mostly
  quarterly cash flows. Traced to empty cells in the extra date columns
  EDGAR adds to a page being read as zero; fixed in `2f488a5` (on 100 S&P
  companies, quarterly operating/investing/financing cash flow zeros fell
  from 190/196/195 to 6/14/10).
- **Jumps of about 1,000×**: thousands read as millions or the reverse.
- **Sign flips**: an outflow that is negative but for a few quarters.

## Why outlier detection across companies didn't work

augurs' outlier detection comes from Grafana's world: a fleet of machines
that should behave alike. If 40 servers behind one load balancer carry the
same load and one doesn't, that one is broken. The method assumes the series
in a group are interchangeable, so a deviation means something.

S&P companies are the opposite, and it flagged 40–80% of them for every item:

1. **They aren't replicas.** A utility, a chip maker and a bank differ in
   growth, seasonality, cyclicality and capital intensity; "unlike the group"
   describes half of them on any measure.
2. **Heavy tails.** Firm sizes and growth rates are fat-tailed (Gibrat's law,
   power laws in firm size); a threshold built for a bell curve fires
   constantly.
3. **The big moves are real events.** Acquisitions, spin-offs, impairments,
   restructurings and accounting changes make sharp breaks. In financial
   data the most dramatic anomalies are usually the business itself.

## The anomalies that matter

What an entrepreneur or investor cares about (a business deteriorating,
aggressive accounting, fraud) doesn't look unusual one series at a time. It
shows as **inconsistency between related figures**:

- **Earnings against cash.** Profits rising while operating cash flow stalls:
  earnings made of accruals rather than cash. High-accrual companies
  subsequently underperform (Sloan 1996, the accrual anomaly).
- **Receivables against revenue.** Receivables growing faster than sales
  (days sales outstanding rising): customers not paying, or revenue booked
  early.
- **Composite scores.** Beneish's M-score (1999; eight ratios, earnings
  manipulation), Altman's Z-score (1968; distress), Piotroski's F-score
  (2000; financial strength among value stocks).
- **The digits themselves.** Benford's law on leading digits, in forensic
  accounting.

These are ratios and identities, joins in the Postgres model
(`store/README.md`).

## Forecasting

Parts are forecastable, though not profitably:

- **Seasonality is real and detectable**: retailers' fourth quarter, travel's
  summer, tax software's April. Worth finding (augurs' seasonality detection,
  MSTL), above all to seasonally adjust series so that real changes, and data
  errors, stand out.
- **Earnings are close to a random walk.** Annual earnings behave like a
  random walk with drift (Ball & Watts 1972); quarterly earnings like a
  seasonal random walk with some short-term persistence, fitted by ARIMA
  models (Foster 1977; Brown & Rozeff 1979). Analysts beat these models only
  modestly, mostly because they know things the history doesn't (Brown &
  Rozeff 1978).

The deeper problem: forecasting fundamentals is not forecasting returns. By
the residual income identity (VALUATION.md), today's price already reflects
the expected path of earnings; only surprises relative to that expectation
move it. A model that forecasts revenue well from seasonality and trend
forecasts what everyone already knows, which is already in the price.

## The efficient market hypothesis, as it evolved

1. **Random walks.** Bachelier (1900) modelled prices on the Paris Bourse as
   a random walk; Kendall (1953) found British stock and commodity prices
   statistically unpredictable; Samuelson, "Proof That Properly Anticipated
   Prices Fluctuate Randomly" (1965), showed why: if prices reflect
   expectations, their changes must be unforecastable.
2. **Fama's formulation** (1965, 1970), in three strengths: weak (past prices
   can't predict future ones), semi-strong (no public information can,
   statements included), strong (not even private information).
3. **Cracks.**
   - Post-earnings-announcement drift: prices keep moving in the direction of
     an earnings surprise for weeks (already in Ball & Brown 1968, the
     founding paper of empirical accounting research; Bernard & Thomas 1989).
   - Excess volatility: prices move far more than the dividends they should
     discount (Shiller 1981).
   - The Grossman–Stiglitz paradox (1980): a perfectly efficient market is
     impossible. If prices revealed everything, nobody would pay to gather
     information, and then prices couldn't reveal it. Some inefficiency must
     remain to pay those who find it.
4. **Anomalies and the factor debate.** Value and size (Fama & French 1992,
   1993), momentum (Jegadeesh & Titman 1993), accruals (Sloan 1996). The
   joint-hypothesis problem: is an anomaly mispricing, or compensation for a
   risk the model misses? Fama read value as risk; behavioral finance (De
   Bondt & Thaler 1985, overreaction) read it as error. Fama and Shiller
   shared the 2013 Nobel, which captures the stalemate.
5. **Now.** Anomalies shrink once published: their returns fall by over half
   afterwards as traders arbitrage them (McLean & Pontiff 2016), so
   efficiency is a process, not a state. Lo's adaptive markets hypothesis
   (2004): efficiency varies with conditions and with who competes, like an
   ecosystem. And the factor zoo: hundreds of published factors, many of them
   artifacts of testing too many hypotheses (Harvey, Liu & Zhu 2016).

## What this leaves for the project

- **Seasonality**: for understanding and cleaning the data, not for trading.
- **Reverse-engineered expectations** rather than forecasts: from price, book
  value and the residual income model, solve for the growth or return on
  equity the market implies, and judge it against the company's own history
  (Rappaport and Mauboussin's "expectations investing"). Not "what will
  revenue be?" but "what must revenue be for this price to make sense?"
- **Ratio and identity checks**: the anomalies that matter, from the same
  data.

## References

- Bachelier, L. (1900). *Théorie de la spéculation*. Annales scientifiques de
  l'École normale supérieure, 17, 21–86.
- Kendall, M. G. (1953). The analysis of economic time-series, part I:
  Prices. *Journal of the Royal Statistical Society A*, 116(1), 11–34.
- Samuelson, P. A. (1965). Proof that properly anticipated prices fluctuate
  randomly. *Industrial Management Review*, 6(2), 41–49.
- Fama, E. F. (1970). Efficient capital markets: A review of theory and
  empirical work. *Journal of Finance*, 25(2), 383–417.
- Ball, R., & Brown, P. (1968). An empirical evaluation of accounting income
  numbers. *Journal of Accounting Research*, 6(2), 159–178.
- Altman, E. I. (1968). Financial ratios, discriminant analysis and the
  prediction of corporate bankruptcy. *Journal of Finance*, 23(4), 589–609.
- Ball, R., & Watts, R. (1972). Some time series properties of accounting
  income. *Journal of Finance*, 27(3), 663–681.
- Foster, G. (1977). Quarterly accounting data: Time-series properties and
  predictive-ability results. *The Accounting Review*, 52(1), 1–21.
- Brown, L. D., & Rozeff, M. S. (1978). The superiority of analyst forecasts
  as measures of expectations. *Journal of Finance*, 33(1), 1–16.
- Brown, L. D., & Rozeff, M. S. (1979). Univariate time-series models of
  quarterly accounting earnings per share. *Journal of Accounting Research*,
  17(1), 179–189.
- Grossman, S. J., & Stiglitz, J. E. (1980). On the impossibility of
  informationally efficient markets. *American Economic Review*, 70(3),
  393–408.
- Shiller, R. J. (1981). Do stock prices move too much to be justified by
  subsequent changes in dividends? *American Economic Review*, 71(3),
  421–436.
- De Bondt, W. F. M., & Thaler, R. (1985). Does the stock market overreact?
  *Journal of Finance*, 40(3), 793–805.
- Bernard, V. L., & Thomas, J. K. (1989). Post-earnings-announcement drift:
  Delayed price response or risk premium? *Journal of Accounting Research*,
  27, 1–36.
- Fama, E. F., & French, K. R. (1992). The cross-section of expected stock
  returns. *Journal of Finance*, 47(2), 427–465.
- Fama, E. F., & French, K. R. (1993). Common risk factors in the returns on
  stocks and bonds. *Journal of Financial Economics*, 33(1), 3–56.
- Jegadeesh, N., & Titman, S. (1993). Returns to buying winners and selling
  losers. *Journal of Finance*, 48(1), 65–91.
- Sloan, R. G. (1996). Do stock prices fully reflect information in accruals
  and cash flows about future earnings? *The Accounting Review*, 71(3),
  289–315.
- Beneish, M. D. (1999). The detection of earnings manipulation. *Financial
  Analysts Journal*, 55(5), 24–36.
- Piotroski, J. D. (2000). Value investing: The use of historical financial
  statement information to separate winners from losers. *Journal of
  Accounting Research*, 38, 1–41.
- Lo, A. W. (2004). The adaptive markets hypothesis. *Journal of Portfolio
  Management*, 30(5), 15–29.
- Harvey, C. R., Liu, Y., & Zhu, H. (2016). … and the cross-section of
  expected returns. *Review of Financial Studies*, 29(1), 5–68.
- McLean, R. D., & Pontiff, J. (2016). Does academic research destroy stock
  return predictability? *Journal of Finance*, 71(1), 5–32.
- Rappaport, A., & Mauboussin, M. J. (2001). *Expectations Investing*.
  Harvard Business School Press.
