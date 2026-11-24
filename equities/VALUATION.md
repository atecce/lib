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

[CAPITAL.md](CAPITAL.md) asks what $`r`$ is underneath: the rate of profit,
the rate of interest and the cost of capital, from Marx and Böhm-Bawerk to
Hayek, Keynes and the Cambridge controversy, and how the interest rate has
formed, layer by layer, in FRED's data since 1857.

## 6. Profitability and returns: ROE as a signal

The residual income model (section 2) runs the other way too. Rearranged,
it says that for a given price-to-book, the market's expected return $`r`$
is higher where expected profitability is higher. That is the case for ROE as
a signal of returns, as against the price-only filter rules of
`FORECASTING.md`, which test the weak form of efficiency and fail it.
Profitability is public, fundamental information (the semi-strong form), and
several studies find it priced, or slow to be.

**From valuation to expected returns.** Fama and French (2006) derive it from
the dividend discount model and clean surplus. Dividing the residual income
form of value by book value:

```math
\frac{M_t}{B_t} \;=\; \frac{\sum_{\tau=1}^{\infty} \mathbb{E}\!\left[Y_{t+\tau} - \Delta B_{t+\tau}\right] / (1+r)^{\tau}}{B_t}
```

with $`Y`$ earnings and $`\Delta B`$ the growth in book value (investment).
Holding the others fixed, a lower price-to-book, higher expected earnings,
or lower expected investment each imply a higher $`r`$. They find all three
in the data: value, profitability and investment each predict returns.

**Profitability, measured.** Novy-Marx (2013) finds gross profitability
(revenue less cost of goods sold, over assets) predicts returns about as
well as book-to-market does, and that the two work together: profitable
firms earn more although they look expensive. "The other side of value":
buying cheap assets and buying productive ones are complementary. Gross
profit, high in the income statement, is less distorted by accounting
choices than net income.

**Factor models built on it.** Fama and French's five-factor model (2015)
adds profitability (RMW, robust minus weak operating profitability) and
investment (CMA, conservative minus aggressive) to market, size and value.
Hou, Xue and Zhang's q-factor model (2015), from the economics of firms'
investment decisions, uses ROE directly: an ROE factor, high minus low
quarterly ROE. Each explains many published anomalies as exposure to
profitability and investment.

**Profitability mean-reverts.** Fama and French (2000) find profitability
reverts toward the mean at about 38% a year, faster from below than from
above, and changes in earnings revert too. An extreme ROE is partly a
forecast that it will fade, so a signal should be ROE against an expected
ROE, not ROE alone.

**Earnings surprises are absorbed slowly.** Ball and Brown (1968), the
founding paper of empirical accounting research, found prices move in the
direction of an earnings surprise before the announcement, and keep drifting
after it. Bernard and Thomas (1989) confirmed the drift after the
announcement (post-earnings-announcement drift, PEAD) and found it strongest
in small firms, consistent with prices underreacting to the news. A jump in
ROE is in good part an earnings surprise.

**What to watch for, using ROE.**

1. **The denominator.** Buybacks shrink book equity, sometimes below zero:
   Apple's ROE of 142% says more about its buybacks than its business, and
   negative equity makes ROE meaningless. Gross profitability, return on
   assets, or return on invested capital carry the same idea with less
   distortion.
2. **Leverage.** By the DuPont decomposition (section 3), ROE rises with
   leverage, so a bank's ROE is mostly leverage. Compare within industries,
   or use ROA.
3. **Expected, not just observed.** "Anomalous" ROE needs a model of expected
   ROE (its mean reversion, its industry); otherwise every growing company
   looks anomalous. (The cross-company outlier detection of `FORECASTING.md`
   failed for this reason.)
4. **Known when.** ROE is known when the filing is, so a test may use only
   what was filed by each date: `fact_first` and `filing.filed_at` in the
   Postgres model.
5. **Published, then arbitraged.** Anomalies weaken once published (McLean
   and Pontiff 2016), and the many tested make multiple-testing corrections
   necessary (`FORECASTING.md`).

**How to test it here.** Each month, with only what had been filed by then,
rank companies by ROE level, by its change over a year, and by an ROE
changepoint (augurs) as an event; form quintile portfolios; measure the next
month's returns and the spread between top and bottom, after costs, across
subperiods, against the filter rules' result. It needs prices for the
universe (Tiingo's free plan allows 500 symbols a month) and, against
survivorship bias, the companies that no longer trade.

## 7. Prices, residuals and retention: Samuelson, Fama, Shiller

The valuation identities above say what a price *is* given expectations.
Three results say how prices *move*, and what the unforecastable part, the
residual, has to look like.

### Samuelson: properly anticipated prices fluctuate randomly

Let $`V`$ be what the stock pays out, discounted, and let the price be its
expectation given the information $`\Phi_t`$ at $`t`$ (constant $`r`$):

```math
P_t \;=\; \mathbb{E}\!\left[\,\sum_{k=1}^{\infty} \frac{D_{t+k}}{(1+r)^{k}} \;\middle|\; \Phi_t \right]
```

Then the discounted price with dividends reinvested is a martingale: by the
law of iterated expectations,

```math
\mathbb{E}\!\left[\,P_{t+1} + D_{t+1} \mid \Phi_t\right] \;=\; (1+r)\,P_t,
\qquad
\varepsilon_{t+1} \;=\; \frac{P_{t+1} + D_{t+1}}{P_t} - (1+r),
\qquad
\mathbb{E}\!\left[\varepsilon_{t+1} \mid \Phi_t\right] = 0
```

The return's surprise $`\varepsilon`$ is uncorrelated with anything known at
$`t`$. Prices change only on news, and news is unforecastable by definition:
randomness is the mark of an informed market, not of a careless one
(Samuelson 1965).

### Fama: efficiency is a statement about residuals

Fama (1970) made this the definition: a market is efficient relative to
information $`\Phi_t`$ if the residuals of its expected-return model are
unforecastable from $`\Phi_t`$,

```math
\mathbb{E}\!\left[\,R_{t+1} - \mathbb{E}_m[R_{t+1} \mid \Phi_t] \;\middle|\; \Phi_t\right] \;=\; 0,
```

with $`\mathbb{E}_m`$ the market's model of expected returns. Weak form:
$`\Phi_t`$ is past prices (the filter rules and serial correlations of
`FORECASTING.md`); semi-strong: all public information (the statements);
strong: everything. Every test is a joint test of efficiency and the model
$`\mathbb{E}_m`$: a residual that looks forecastable may be mispricing or a
wrong model of risk (the joint-hypothesis problem).

### Retention: residual income and the growth it buys

Clean surplus (section 2) splits earnings between payout and retention,
$`B_t = B_{t-1} + E_t - D_t`$. With a retention rate $`b = 1 - D_t/E_t`$ and
constant $`\mathrm{ROE}`$, book value, earnings and dividends all grow at

```math
g \;=\; b \cdot \mathrm{ROE}
```

and the Gordon model, $`P = D_1/(r - g)`$, becomes

```math
P_0 \;=\; \frac{(1-b)\,\mathrm{ROE}\;B_0}{r - b\,\mathrm{ROE}},
\qquad
\frac{P_0}{B_0} \;=\; \frac{\mathrm{ROE} - g}{r - g},
```

the justified price-to-book of section 2 again. Retaining more raises value
only if $`\mathrm{ROE} > r`$: differentiating,

```math
\frac{\partial P_0}{\partial b} \;=\; \frac{\mathrm{ROE}\,B_0\,(\mathrm{ROE} - r)}{(r - b\,\mathrm{ROE})^2},
```

positive when retained earnings earn more than shareholders require, zero
when they earn exactly $`r`$ (where payout policy is irrelevant, Miller and
Modigliani's result), negative below. Each dollar retained adds a dollar of
book value; it adds more than a dollar of market value only by earning
residual income, $`(\mathrm{ROE} - r)\,B`$. Market minus book is the present
value of the residual income retention is expected to buy.

### Shiller: prices move too much

Define the ex post rational price $`P^*_t`$, the present value of the
dividends that were actually paid. Rational expectations make the price its
forecast, so the forecast error is uncorrelated with the forecast:

```math
P^*_t \;=\; P_t + u_t, \qquad \operatorname{Cov}(P_t, u_t) = 0
\quad\Longrightarrow\quad
\operatorname{Var}(P^*_t) \;=\; \operatorname{Var}(P_t) + \operatorname{Var}(u_t) \;\ge\; \operatorname{Var}(P_t).
```

A forecast can't vary more than what it forecasts. Shiller (1981) found the
reverse in a century of US data: detrended prices several times as volatile
as $`P^*`$, the "excess volatility" that was one of the first cracks in the
efficient market view. In residual income terms the same bound applies to
$`M - B`$ against the present value of the residual income actually earned:
market-to-book should move less than realized excess profitability does.

The log-linear version (Campbell and Shiller 1988) says where the movement
must go. With $`p`$ and $`d`$ log price and dividend and $`\rho`$ slightly
below 1,

```math
p_t - d_t \;\approx\; \text{const} + \sum_{j=0}^{\infty} \rho^{j}\left(\Delta d_{t+1+j} - r_{t+1+j}\right),
```

so a high price relative to dividends must be followed either by fast
dividend growth or by low returns. Empirically it's mostly low returns:
valuation ratios forecast returns over years (Fama and French 1988;
Campbell and Shiller 1988), while daily and monthly returns stay nearly
unforecastable. That reconciles Samuelson and Shiller, and Fama and Shiller
shared the 2013 Nobel Prize for it.

### For this project

The dashboard already holds both sides: market value and book value
(section 4), so $`M - B`$, and from the statements the residual income each
year, $`E_t - r\,B_{t-1}`$, retained or paid out. Shiller's bound and Black's
factor of two ("Noise", 1986) are both testable against them: how far
$`M - B`$ wanders from the present value of residual income later earned.

## 8. Entrepreneurial anomalies: persistent residual income

Notes from 2026-11-23, after `FORECASTING.md`'s outlier scan. Run across
companies on raw statement lines, augurs flagged 40–80% of them: what it
found were accounting quirks (buybacks shrinking equity, acquisitions,
write-downs) and companies that are simply different (a bank against a chip
maker). Those are *statistical* anomalies: values far from their group or
their own history. The anomaly an entrepreneur recognizes is an *economic*
one, and it has a precise form in the residual income model.

### Two kinds of anomaly

Residual income (abnormal earnings, section 2) is what a business earns
beyond the return its capital requires:

```math
x^a_t \;=\; E_t - r\,B_{t-1} \;=\; (\mathrm{ROE}_t - r)\,B_{t-1}
```

and market minus book is what the market expects of it, discounted:

```math
M_t - B_t \;=\; \sum_{\tau=1}^{\infty} \frac{\mathbb{E}_t\!\left[x^a_{t+\tau}\right]}{(1+r)^{\tau}}
```

An entrepreneurial anomaly is $`\mathrm{ROE} > r`$, kept up: a business
earning more than its capital should. That it's an anomaly at all is a claim
about competition, which says it shouldn't last.

### Competition predicts decay

Rivals copy, enter and undercut, so excess returns should shrink toward
zero. Ohlson's dynamics (section 2) build that in: 
$`x^a_{t+1} = \omega\, x^a_t + \dots`$ with $`\omega < 1`$. The evidence agrees on the direction:

- **Profitability reverts.** Fama and French (2000) find it moves toward the
  mean about 38% a year, faster when far below it.
- **Abnormal earnings persist, partly.** Dechow, Hutton and Sloan (1999)
  estimate $`\omega \approx 0.62`$ for US companies, varying with
  industry, accruals and dividends.
- **Some profits don't converge.** The persistence-of-profits literature
  (Mueller 1986) finds a minority of firms whose excess returns stay well
  above the norm for decades.

So the economic anomaly is residual income that **decays more slowly than
competition predicts**, or that rises when it should fall. In the model's
terms: a company whose own $`\omega`$ is well above its industry's.

### The economists of it

- **Schumpeter (1911, translated 1934):** entrepreneurial profit is the
  temporary monopoly an innovation earns until imitation erodes it, through
  "creative destruction". Residual income is that profit; $`\omega`$ is how
  slowly the imitators arrive.
- **Knight (1921):** profit, as distinct from interest and from a return for
  bearing measurable risk, rewards facing true uncertainty, which can't be
  insured or priced in advance. Whatever excess over $`r`$ remains once risk
  is priced is Knight's profit.
- **Kirzner (1973):** the entrepreneur is alert to opportunities others have
  missed, and by acting on them moves the market toward them. In a market
  for shares, that means seeing a price that is wrong.
- **Mauboussin and Johnson (1997):** the "competitive advantage period", how
  long a company can keep returns above its cost of capital, is the value
  question, and the market's price states its estimate of it.

### What the price says the persistence is

Two ways of reading that estimate from the dashboard's numbers, with
$`R = 1 + r`$.

**Implied decay.** In Ohlson's model with no other information
($`\nu = 0`$), $`M - B = \frac{\omega}{R - \omega}\, x^a`$. Solved for
$`\omega`$:

```math
\omega_{\text{implied}} \;=\; \frac{R\,(M_t - B_t)}{x^a_t + (M_t - B_t)}
```

meaningful where $`x^a_t > 0`$ and $`M_t > B_t`$. A value near 1 means the
market prices the advantage as lasting; near 0, as gone next year.

**Implied advantage period.** If residual income stays at today's level for
$`T`$ years and then stops, $`M - B = x^a \,(1 - R^{-T}) / r`$, so

```math
T_{\text{implied}} \;=\; -\,\frac{\ln\!\left(1 - r\,(M_t - B_t)/x^a_t\right)}{\ln R}
```

and where $`r\,(M - B) \ge x^a`$ the market is pricing more than a
perpetuity of today's excess: growth in it.

### A detector

The three steps, each from data already in Postgres:

1. **Residual income per company and year**, $`E_t - r\,B_{t-1}`$, with
   $`r`$ from section 5 (the risk-free rate plus a premium, or an implied
   cost of capital). Compare within industries (SIC codes from EDGAR), since
   leverage alone raises a bank's ROE.
2. **Its persistence against competition's.** Estimate each company's
   $`\omega`$ by regressing $`x^a_{t+1}`$ on $`x^a_t`$, both divided by the
   same total assets $`A_{t-1}`$ (a common deflator, so size drops out and
   $`\omega`$ keeps its meaning), and the industry's the same way, pooled
   over its companies (below). The anomaly is a company $`\omega`$
   well above its industry's, or a *changepoint* in residual income: an
   advantage forming or collapsing. This is where augurs' changepoint
   detection belongs: on the economically meaningful series, not on raw
   statement lines.
3. **Against the price.** Compare the realized persistence with
   $`\omega_{\text{implied}}`$. For an investor the anomaly that pays is
   persistence *greater than priced*: the market underrating how long a
   moat holds. That is the profitability premium of section 6 restated
   (Novy-Marx 2013; Fama and French 2015), and Frankel and Lee (1998) find
   that residual income value over price, with analysts' forecasts, predicts
   returns. The backtest: sort on 
   $`\omega_{\text{realized}} - \omega_{\text{implied}}`$ known at each filing date, and measure the
   returns that follow.

### Industries

An industry's $`\omega`$ is the same regression with every company-year of
the industry's companies in it, one pooled fit:

```math
\frac{x^a_{i,t+1}}{A_{i,t-1}} \;=\; \omega_{\text{industry}}\,\frac{x^a_{i,t}}{A_{i,t-1}} + \varepsilon_{i,t+1}
```

(Fama and French 2000 do the cross-sectional version: each year, the change
in profitability regressed on its distance from the expected level, averaged
over years.) Which companies form an industry is a classification:

- **SIC**, the Standard Industrial Classification (US, 1937; four digits:
  division, major group 35, industry group 357, industry 3571). Frozen
  since 1987 for most uses, but the SEC still assigns one to every filer, so
  EDGAR's submissions carry it: in Postgres as `issuer.sic` (migration 009,
  filled by `universe | store-sic`). Free, and on every company here.
- **NAICS**, the North American Industry Classification System (1997),
  which replaced SIC for US, Canadian and Mexican statistics; six digits,
  better for services and technology. Not in EDGAR's filer data.
- **GICS**, the Global Industry Classification Standard (MSCI and S&P,
  1999): 11 sectors down to 163 sub-industries, by what drives revenue; what
  the S&P 500's sectors are. Proprietary.
- **Fama–French industries**: 5, 10, 12, 17, 30, 38, 48 or 49 groups built
  from SIC codes, published with the mappings on Kenneth French's data
  library; the convention in asset pricing papers, so results compare. A
  mapping table from SIC is all they take.
- **Text-based industries** (Hoberg and Phillips 2016): each company's
  peers from the similarity of the product descriptions in its 10-K, redrawn
  every year. Industries this way follow the companies, which is the point
  for entrepreneurs: a new good redraws who competes with whom, and a fixed
  code can't (see `aapl/README.md`, "The wrong baseline").
- **The whole market**: no classification at all, the null that assumes
  nothing about who competes with whom. Fama and French (2000) measure it
  this way.
- **Japan and Taiwan**: EDINET and MOPS carry their exchanges' own industry
  codes (TSE's 33 sectors; TWSE's industry categories), to map when those
  companies join the comparison.

`backtest/examples/persistence.rs` groups by the first three SIC digits (the
industry group; `--digits` changes it). For Apple that is computer and
office equipment, 357; `aapl/README.md` is the worked example.

**The data is long enough for industries, barely for companies.** XBRL
starts in 2009, so about 16 annual observations per company (60-odd
quarters, seasonal): a company's own $`\omega`$ is noisy, with a standard
error around $`\sqrt{(1-\omega^2)/16}`$, about 0.2 at $`\omega = 0.6`$.
Pooled by industry it is well determined. The √N argument of `DATA.md` again:
breadth where length can't help.

### Caveats

- **Book value isn't capital.** Research, brands and software expensed as
  incurred never reach $`B`$, so their owners show a high ROE that is partly
  an accounting artifact (and a low one while they invest). Persistence is
  more robust to this than the level, but not immune.
- **Buybacks and negative equity.** Repurchases shrink $`B`$ and inflate
  ROE; with negative equity (as at some large repurchasers) ROE is
  meaningless and residual income must be scaled by assets or sales instead.
- **The cost of equity is an estimate.** A wrong $`r`$ moves every
  company's $`x^a`$ by $`\Delta r \cdot B`$; comparisons within an industry
  and over time are less sensitive than levels.
- **Point in time.** Each value has to be what was known then: the filing
  date, not the period end, and the first report, not the restated one
  (`fact_first` in Postgres).
- **Survivorship.** Moats that held are overrepresented among today's
  tickers; the CIK universe of tickerless filers (delisted, acquired,
  failed) is what keeps the collapses in the sample.

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
- Ball, R., & Brown, P. (1968). An empirical evaluation of accounting income
  numbers. *Journal of Accounting Research*, 6(2), 159–178. Prices anticipate
  earnings surprises and keep drifting after them.
- Bernard, V. L., & Thomas, J. K. (1989). Post-earnings-announcement drift:
  Delayed price response or risk premium? *Journal of Accounting Research*,
  27 (Supplement), 1–36. The drift is underreaction, strongest in small firms.
- Fama, E. F., & French, K. R. (2000). Forecasting profitability and
  earnings. *Journal of Business*, 73(2), 161–175. Profitability reverts
  toward the mean, about 38% a year.
- Fama, E. F., & French, K. R. (2006). Profitability, investment and average
  returns. *Journal of Financial Economics*, 82(3), 491–518. From the dividend
  discount model: given price-to-book, higher expected profitability and lower
  investment mean higher expected returns.
- Novy-Marx, R. (2013). The other side of value: The gross profitability
  premium. *Journal of Financial Economics*, 108(1), 1–28. Gross profit over
  assets predicts returns as well as book-to-market, and with it.
- Fama, E. F., & French, K. R. (2015). A five-factor asset pricing model.
  *Journal of Financial Economics*, 116(1), 1–22. Market, size, value,
  profitability (RMW) and investment (CMA).
- Hou, K., Xue, C., & Zhang, L. (2015). Digesting anomalies: An investment
  approach. *Review of Financial Studies*, 28(3), 650–705. The q-factor model,
  with an ROE factor.
- Samuelson, P. A. (1965). Proof that properly anticipated prices fluctuate
  randomly. *Industrial Management Review*, 6(2), 41–49. Prices that are
  expectations follow a martingale.
- Fama, E. F. (1970). Efficient capital markets: A review of theory and
  empirical work. *Journal of Finance*, 25(2), 383–417. Efficiency as
  unforecastable residuals; weak, semi-strong and strong forms.
- Miller, M. H., & Modigliani, F. (1961). Dividend policy, growth, and the
  valuation of shares. *Journal of Business*, 34(4), 411–433. Payout policy
  doesn't change value; investment does.
- Shiller, R. J. (1981). Do stock prices move too much to be justified by
  subsequent changes in dividends? *American Economic Review*, 71(3),
  421–436. The variance bound, and its violation.
- Campbell, J. Y., & Shiller, R. J. (1988). The dividend-price ratio and
  expectations of future dividends and discount factors. *Review of
  Financial Studies*, 1(3), 195–228. The log-linear identity.
- Fama, E. F., & French, K. R. (1988). Dividend yields and expected stock
  returns. *Journal of Financial Economics*, 22(1), 3–25. Valuation ratios
  forecast returns over long horizons.
- Black, F. (1986). Noise. *Journal of Finance*, 41(3), 529–543. Noise
  makes trading possible and prices imperfect: within a factor of two of
  value.
- McLean, R. D., & Pontiff, J. (2016). Does academic research destroy stock
  return predictability? *Journal of Finance*, 71(1), 5–32. Anomaly returns
  fall by over half after publication.
- Schumpeter, J. A. (1934). *The Theory of Economic Development*. Harvard
  University Press (German original 1911). Entrepreneurial profit as the
  temporary monopoly of innovation, eroded by imitation.
- Knight, F. H. (1921). *Risk, Uncertainty and Profit*. Houghton Mifflin.
  Profit as the reward for bearing uninsurable uncertainty.
- Kirzner, I. M. (1973). *Competition and Entrepreneurship*. University of
  Chicago Press. The entrepreneur as alert to missed opportunities.
- Mueller, D. C. (1986). *Profits in the Long Run*. Cambridge University
  Press. Some firms' excess profits persist for decades.
- Mauboussin, M. J., & Johnson, P. (1997). Competitive advantage period: The
  neglected value driver. *Financial Management*, 26(2), 67–74. How long
  returns exceed the cost of capital, as priced.
- Frankel, R., & Lee, C. M. C. (1998). Accounting valuation, market
  expectation, and cross-sectional stock returns. *Journal of Accounting and
  Economics*, 25(3), 283–319. Residual income value over price predicts
  returns.
- Hayek, F. A. (2002). Competition as a discovery procedure. *Quarterly
  Journal of Austrian Economics*, 5(3), 9–23 (German original 1968).
  Competition finds out what nobody knew beforehand, including who
  competes with whom.
- Hoberg, G., & Phillips, G. (2016). Text-based network industries and
  endogenous product differentiation. *Journal of Political Economy*,
  124(5), 1423–1465. Industries from 10-K product descriptions, redrawn
  yearly.
- Dechow, P. M., Hutton, A. P., & Sloan, R. G. (1999). An empirical
  assessment of the residual income valuation model. *Journal of Accounting
  and Economics*, 26(1–3), 1–34. Abnormal earnings persist with
  $`\omega \approx 0.62`$.

The attribution to Preinreich, Edwards and Bell, and Peasnell before Ohlson is
the usual one in the accounting literature; the model is often called
Edwards–Bell–Ohlson (EBO).
