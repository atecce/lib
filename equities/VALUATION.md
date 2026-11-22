# Book value, market value, and the residual income model

Notes from 2026-11-21 on the valuation row of `equities/grafana/equities.json`
and `equities/src/valuation.rs`. The math is LaTeX, which GitHub renders in
Markdown.

## Notation

| Symbol | Meaning |
|---|---|
| $M_t$ | market value of equity (market cap) at time $t$ |
| $B_t$ | book value of equity (stockholders' equity) |
| $E_t$ | earnings (net income) over period $t$ |
| $D_t$ | net payout to shareholders over period $t$: dividends plus buybacks, less issuance |
| $r$ | cost of equity: the return shareholders require |
| $g$ | growth rate |
| $\mathrm{ROE}_t = E_t / B_{t-1}$ | return on equity |

## 1. The identity

Multiply and divide by earnings:

$$
\frac{M}{B} \;=\; \frac{M}{E} \cdot \frac{E}{B}
\qquad\Longleftrightarrow\qquad
\underbrace{\frac{M}{B}}_{\text{price-to-book}}
\;=\; \frac{\overbrace{E/B}^{\text{return on equity}}}{\underbrace{E/M}_{\text{earnings yield}}}
$$

Price-to-book is P/E times ROE, or ROE divided by earnings yield (the inverse
of P/E). As written this is algebra, true by construction. The dashboard's
version is approximate because its ROE uses equity averaged over the year.

## 2. The theorem behind it: residual income

What makes the identity meaningful is a valuation result that says *why*
price-to-book depends on ROE. It follows from two assumptions.

**Value is the present value of payouts** (the dividend discount model):

$$
V_0 \;=\; \sum_{t=1}^{\infty} \frac{D_t}{(1+r)^t}
$$

**Clean surplus:** book value changes only by earnings kept, the stock-and-flow
link between the balance sheet and income statement:

$$
B_t \;=\; B_{t-1} + E_t - D_t
$$

Substituting $D_t = E_t - (B_t - B_{t-1})$ into the first, and assuming book
value grows more slowly than $(1+r)^t$, gives the **residual income model**:

$$
V_0 \;=\; B_0 \;+\; \sum_{t=1}^{\infty} \frac{E_t - r\,B_{t-1}}{(1+r)^t}
\;=\; B_0 \;+\; \sum_{t=1}^{\infty} \frac{(\mathrm{ROE}_t - r)\,B_{t-1}}{(1+r)^t}
$$

The term $E_t - r\,B_{t-1}$ is **residual income** or **abnormal earnings**:
profit beyond what the book capital should earn. Dividing by $B_0$:

$$
\frac{V_0}{B_0} \;=\; 1 \;+\; \sum_{t=1}^{\infty} \frac{(\mathrm{ROE}_t - r)}{(1+r)^t}\,\frac{B_{t-1}}{B_0}
$$

So price-to-book is above 1 exactly when the market expects ROE above the cost
of equity, equal to 1 when it expects them equal, and below 1 when it expects
the capital to earn less than shareholders require.

Real accounting bends clean surplus: items that bypass the income statement
(other comprehensive income, such as currency translation) change book value
without passing through earnings.

### Constant ROE and growth: justified price-to-book

With constant $\mathrm{ROE}$ and book value growing at $g < r$:

$$
\frac{M}{B} \;=\; \frac{\mathrm{ROE} - g}{r - g}
$$

This is the Gordon growth model (value of growing payouts) written in terms
of book value.

### Ohlson's linear information dynamics

Ohlson (1995) closed the model by assuming abnormal earnings
$x^a_t = E_t - r\,B_{t-1}$ decay at rate $\omega$, driven also by other
information $\nu_t$ that persists at rate $\gamma$:

$$
x^a_{t+1} = \omega\, x^a_t + \nu_t + \varepsilon_{1,t+1},
\qquad
\nu_{t+1} = \gamma\, \nu_t + \varepsilon_{2,t+1}
$$

with $0 \le \omega, \gamma < 1$. With $R = 1 + r$, value is then linear in
book value, abnormal earnings and other information:

$$
P_t \;=\; B_t + \alpha_1\, x^a_t + \alpha_2\, \nu_t,
\qquad
\alpha_1 = \frac{\omega}{R - \omega},
\qquad
\alpha_2 = \frac{R}{(R - \omega)(R - \gamma)}
$$

## 3. Taking ROE apart: the DuPont decomposition

$$
\mathrm{ROE} \;=\; \frac{E}{B}
\;=\; \underbrace{\frac{E}{\text{Revenue}}}_{\text{profit margin}}
\cdot \underbrace{\frac{\text{Revenue}}{\text{Assets}}}_{\text{asset turnover}}
\cdot \underbrace{\frac{\text{Assets}}{B}}_{\text{leverage}}
$$

This is where the balance sheet and income statement detail feed in: ROE is
high through margins (Apple, Nvidia), turnover (retailers), or leverage
(banks, and car makers through their lending arms).

## 4. On the dashboard (2026-11-20)

| Company | ROE | Earnings yield | ROE ÷ yield | Price-to-book |
|---|---|---|---|---|
| GM | 6.5% | 6.3% | 1.03 | 1.06 |
| Berkshire | 10.6% | 6.4% | 1.66 | 1.59 |
| TSMC | 36% | 3.3% | ~11 | 9.7 |
| Nvidia | 117% | 6.4% | ~18 | 13 |
| Apple | 142% | 2.7% | ~53 | 44 |

GM earns about its earnings yield and trades at book. Apple's ROE is high
partly because buybacks have shrunk its book value: payouts reduce $B$ through
clean surplus.

When book value says little: for businesses whose assets are mostly financial
and near market value (banks, insurers, real estate trusts, car makers'
lending arms), $B$ is a strong anchor and price-to-book the standard measure.
For asset-light companies, most of what they're worth never appears on a
balance sheet (internally built software, brands, research), so nearly all of
$M - B$ is the market's forecast of residual income.

## References

- Preinreich, G. A. D. (1938). Annual survey of economic theory: The theory of
  depreciation. *Econometrica*, 6(3), 219–241.
- Edwards, E. O., & Bell, P. W. (1961). *The Theory and Measurement of Business
  Income*. University of California Press.
- Peasnell, K. V. (1982). Some formal connections between economic values and
  yields and accounting numbers. *Journal of Business Finance & Accounting*,
  9(3), 361–381.
- Ohlson, J. A. (1995). Earnings, book values, and dividends in equity
  valuation. *Contemporary Accounting Research*, 11(2), 661–687.
- Feltham, G. A., & Ohlson, J. A. (1995). Valuation and clean surplus
  accounting for operating and financial activities. *Contemporary Accounting
  Research*, 11(2), 689–731.
- Gordon, M. J. (1959). Dividends, earnings, and stock prices. *Review of
  Economics and Statistics*, 41(2), 99–105.

The attribution to Preinreich, Edwards and Bell, and Peasnell before Ohlson is
the usual one in the accounting literature; the model is often called
Edwards–Bell–Ohlson (EBO).
