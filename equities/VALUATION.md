# Book value, market value, and the residual income model

Notes from 2026-11-21 on the valuation row of `equities/grafana/equities.json`
and `equities/src/valuation.rs`. The math is LaTeX, fenced the way GitHub
asks (``$`…`$`` inline, ```` ```math ```` blocks) so Markdown leaves its
underscores alone.

## Notation

| Symbol | Meaning |
|---|---|
| $`M_t`$ | market value of equity (market cap) at time $`t`$ |
| $`B_t`$ | book value of equity (stockholders' equity) |
| $`E_t`$ | earnings (net income) over period $`t`$ |
| $`D_t`$ | net payout to shareholders over period $`t`$: dividends plus buybacks, less issuance |
| $`r`$ | cost of equity: the return shareholders require |
| $`g`$ | growth rate |
| $`\mathrm{ROE}_t = E_t / B_{t-1}`$ | return on equity |

## 1. The identity

Multiply and divide by earnings:

```math
\frac{M}{B} \;=\; \frac{M}{E} \cdot \frac{E}{B}
\qquad\Longleftrightarrow\qquad
\underbrace{\frac{M}{B}}_{\text{price-to-book}}
\;=\; \frac{\overbrace{E/B}^{\text{return on equity}}}{\underbrace{E/M}_{\text{earnings yield}}}
```

Price-to-book is P/E times ROE, or ROE divided by earnings yield (the inverse
of P/E). As written this is algebra, true by construction. The dashboard's
version is approximate because its ROE uses equity averaged over the year.

## 2. The theorem behind it: residual income

What makes the identity meaningful is a valuation result that says *why*
price-to-book depends on ROE. It follows from two assumptions.

**Value is the present value of payouts** (the dividend discount model):

```math
V_0 \;=\; \sum_{t=1}^{\infty} \frac{D_t}{(1+r)^t}
```

**Clean surplus:** book value changes only by earnings kept, the stock-and-flow
link between the balance sheet and income statement:

```math
B_t \;=\; B_{t-1} + E_t - D_t
```

Substituting $`D_t = E_t - (B_t - B_{t-1})`$ into the first, and assuming book
value grows more slowly than $`(1+r)^t`$, gives the **residual income model**:

```math
V_0 \;=\; B_0 \;+\; \sum_{t=1}^{\infty} \frac{E_t - r\,B_{t-1}}{(1+r)^t}
\;=\; B_0 \;+\; \sum_{t=1}^{\infty} \frac{(\mathrm{ROE}_t - r)\,B_{t-1}}{(1+r)^t}
```

The term $`E_t - r\,B_{t-1}`$ is **residual income** or **abnormal earnings**:
profit beyond what the book capital should earn. Dividing by $`B_0`$:

```math
\frac{V_0}{B_0} \;=\; 1 \;+\; \sum_{t=1}^{\infty} \frac{(\mathrm{ROE}_t - r)}{(1+r)^t}\,\frac{B_{t-1}}{B_0}
```

So price-to-book is above 1 exactly when the market expects ROE above the cost
of equity, equal to 1 when it expects them equal, and below 1 when it expects
the capital to earn less than shareholders require.

Real accounting bends clean surplus: items that bypass the income statement
(other comprehensive income, such as currency translation) change book value
without passing through earnings.

### Constant ROE and growth: justified price-to-book

With constant $`\mathrm{ROE}`$ and book value growing at $`g < r`$:

```math
\frac{M}{B} \;=\; \frac{\mathrm{ROE} - g}{r - g}
```

This is the Gordon growth model (value of growing payouts) written in terms
of book value.

### Ohlson's linear information dynamics

Ohlson (1995) closed the model by assuming abnormal earnings
$`x^a_t = E_t - r\,B_{t-1}`$ decay at rate $`\omega`$, driven also by other
information $`\nu_t`$ that persists at rate $`\gamma`$:

```math
x^a_{t+1} = \omega\, x^a_t + \nu_t + \varepsilon_{1,t+1},
\qquad
\nu_{t+1} = \gamma\, \nu_t + \varepsilon_{2,t+1}
```

with $`0 \le \omega, \gamma < 1`$. With $`R = 1 + r`$, value is then linear in
book value, abnormal earnings and other information:

```math
P_t \;=\; B_t + \alpha_1\, x^a_t + \alpha_2\, \nu_t,
\qquad
\alpha_1 = \frac{\omega}{R - \omega},
\qquad
\alpha_2 = \frac{R}{(R - \omega)(R - \gamma)}
```

## 3. Taking ROE apart: the DuPont decomposition

```math
\mathrm{ROE} \;=\; \frac{E}{B}
\;=\; \underbrace{\frac{E}{\text{Revenue}}}_{\text{profit margin}}
\cdot \underbrace{\frac{\text{Revenue}}{\text{Assets}}}_{\text{asset turnover}}
\cdot \underbrace{\frac{\text{Assets}}{B}}_{\text{leverage}}
```

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
partly because buybacks have shrunk its book value: payouts reduce $`B`$ through
clean surplus.

When book value says little: for businesses whose assets are mostly financial
and near market value (banks, insurers, real estate trusts, car makers'
lending arms), $`B`$ is a strong anchor and price-to-book the standard measure.
For asset-light companies, most of what they're worth never appears on a
balance sheet (internally built software, brands, research), so nearly all of
$`M - B`$ is the market's forecast of residual income.

## 5. Earnings yield and the cost of equity

### What the dashboard measures

```math
\text{earnings yield}_t \;=\; \frac{E^{\text{TTM}}_t}{M_t}
```

net income over the last four quarters (as first reported) over the market
cap that day (`equities/src/valuation.rs`): the inverse of the trailing P/E.
It is *observed*: past accounting earnings against today's price.

### What $`r`$ is

$`r`$ is exogenous to the residual income model (an input, not something it
solves for), but it is not the Treasury rate. It is the **cost of equity**:
the return investors could expect elsewhere on something *equally risky*,
and so the opportunity cost of the shareholders' capital. The Treasury rate
is its floor, and a risk premium sits on top. In the capital asset pricing
model (CAPM):

```math
r \;=\; r_f + \beta\,\bigl(\mathbb{E}[R_m] - r_f\bigr)
```

with $`r_f`$ the risk-free (Treasury) rate, $`\mathbb{E}[R_m] - r_f`$ the
equity risk premium, and $`\beta`$ how strongly the stock moves with the
market. With Treasuries around 4% and a premium of 4–6%, a typical stock's
$`r`$ is around 8–10%. A company earning only the Treasury rate on its book
value would trade *below* book: its shareholders bear risk its bondholders
don't. Unlike earnings yield, $`r`$ is *expected* (forward-looking) and
cannot be observed.

### How they relate

From the justified price-to-book, $`M/B = (\mathrm{ROE} - g)/(r - g)`$:

```math
\frac{E}{M} \;=\; \frac{\mathrm{ROE}}{M/B} \;=\; \mathrm{ROE}\cdot\frac{r-g}{\mathrm{ROE}-g}
```

1. **$`\mathrm{ROE} = r`$:** then $`E/M = r`$ whatever the growth. Growth
   that only earns the cost of capital adds no value, so price is book and
   earnings yield is $`r`$. GM is close to this case (ROE 6.5%, earnings
   yield 6.3%, price-to-book 1.06). Read literally that makes the market's
   $`r`$ for GM about 6.5%, low for a car maker; more likely the market
   expects its ROE to fall.
2. **$`\mathrm{ROE} > r`$, with growth:** then $`E/M < r`$. The price
   includes future excess returns, so today's earnings look small against
   it. Apple's 2.7% earnings yield doesn't mean its shareholders expect
   2.7%; they are paying for decades of very high ROE.
3. **$`\mathrm{ROE} < r`$:** then $`E/M > r`$, and the company trades below
   book.

So earnings yield is $`r`$ blurred by expected growth in excess returns.
Comparing a market's earnings yield directly with Treasury yields (the
"Fed model") treats the first case as if it always held. Asness (2003)
criticizes it for a second reason too: earnings grow with inflation, while
a bond's yield is fixed in nominal terms.

### The implied cost of capital

Since $`r`$ can't be observed, it can be backed out instead: take the price,
book value and an earnings forecast, and solve the residual income model for
the $`r`$ that reproduces the price (Gebhardt, Lee and Swaminathan 2001;
Claus and Thomas 2001). Under constant ROE and growth:

```math
r \;=\; g + \frac{\mathrm{ROE} - g}{M/B}
```

This, not the earnings yield, is the number to compare with the Treasury
rate: their difference is the market's implied equity risk premium for the
company. It depends on the growth assumed, and on ROE staying where it is.

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
- Sharpe, W. F. (1964). Capital asset prices: A theory of market equilibrium
  under conditions of risk. *Journal of Finance*, 19(3), 425–442.
- Claus, J., & Thomas, J. (2001). Equity premia as low as three percent?
  Evidence from analysts' earnings forecasts for domestic and international
  stock markets. *Journal of Finance*, 56(5), 1629–1666.
- Gebhardt, W. R., Lee, C. M. C., & Swaminathan, B. (2001). Toward an
  implied cost of capital. *Journal of Accounting Research*, 39(1), 135–176.
- Asness, C. S. (2003). Fight the Fed model. *Journal of Portfolio
  Management*, 30(1), 11–24.

The attribution to Preinreich, Edwards and Bell, and Peasnell before Ohlson is
the usual one in the accounting literature; the model is often called
Edwards–Bell–Ohlson (EBO).
