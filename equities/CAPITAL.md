# Capital, interest and r

The residual income model ([VALUATION.md](VALUATION.md)) runs on one number
it never explains: $`r`$, the return the shareholders' capital "should" earn.
Book value earning exactly $`r`$ is worth book; earning more, it is worth
more, and the excess, $`\mathrm{ROE} - r`$, is what section 8 tracks as it
decays or persists. So what is $`r`$? Three different things have gone by
that letter, and the argument over how they relate is the oldest one in
economics:

1. **The rate of profit:** what capital earns in production, as a share of
   the capital advanced (Marx, and the Cambridge economists after Sraffa).
2. **The rate of interest:** the price of lending money for a time, set in a
   market, and since 1914 partly by a central bank.
3. **The cost of capital:** the return given up by putting money here
   rather than in the next best thing of equal risk (modern finance).

The model's $`r`$ is the third. The first is what $`\mathrm{ROE}`$ measures.
The second is the one thing among them you can see in a price. This
document is about how the three connect: the theories (from Marx to Hayek
and his critics), then the data, 1857–2025, from FRED.

## Marx: the rate of profit, and its equalization

In *Capital*, money becomes capital when it is advanced to come back larger
(M–C–M′). Volume III defines the rate of profit (written $`p'`$ there; $`r`$
is Sraffa's and Piketty's letter) as surplus value over the whole capital
advanced, constant and variable: $`p' = s/(c+v)`$.

Two parts of Volume III bear directly on the residual income model:

- **Equalization (Part II).** Capital leaves industries with low rates of
  profit and floods into those with high ones, until each earns the
  *general* rate of profit and goods sell at "prices of production" rather
  than at their values. The general rate is the "normal" return; anything
  above it is *surplus profit*, and competition erodes it. This is
  $`\mathrm{ROE} - r`$ decaying toward zero: Ohlson's $`\omega < 1`$ is
  Marx's equalization, measured.
- **Surplus profit that doesn't erode (Part VI, chapter 38).** Marx's
  example is a factory driven by a waterfall: its costs are lower than its
  steam-driven rivals', and its surplus profit persists because the
  waterfall can't be reproduced by capital; it is a rent. In section 8's
  terms, a waterfall is $`\omega`$ near 1. Apple's realized
  $`\omega = 1.12`$ and Berkshire's 0.92 over 1980–98 are the cases where
  the question "what is the waterfall?" has to be answered.

And interest, for Marx, is not a rate of its own. It is a slice of profit,
paid by the capitalist who uses money to the one who lends it (Part V).
In chapter 22 he argues there is no *natural* rate of interest in the
sense that there is a natural (general) rate of profit: the split between
lender and borrower is settled by supply and demand for loan capital, and
by custom. The rate of profit is the bound; the rate of interest is
whatever the money market makes it below that.

## Böhm-Bawerk: interest without exploitation

Eugen von Böhm-Bawerk's *Capital and Interest* (1884) took apart every
earlier theory of interest, Marx's exploitation theory included, and his
*Positive Theory of Capital* (1889) put one in its place. Interest is an
*agio*: present goods are worth more than the same goods in the future, for
three reasons:

1. people expect to be better provided for later;
2. they systematically underrate future wants;
3. present goods are technically superior: they can be put into longer,
   "roundabout" processes of production, which yield more.

So interest would exist in any economy that plans across time, socialist
or not; it is not something taken from labor but a price of time. The
third reason led him to measure capital by an *average period of
production*, and that is where the trouble started.

## Wicksell: two rates

Knut Wicksell (*Interest and Prices*, 1898) split the rate in two. The
**natural rate** is the return expected on new capital: the rate that would
rule if capital were lent in kind, without money. The **money rate** is what
banks charge. When banks lend below the natural rate, borrowing to invest
pays, spending outruns saving and prices rise, cumulatively, for as long as
the gap lasts. Above it, they fall. The money rate is visible; the natural
rate is not.

## Hayek: the structure of production

Hayek took Wicksell's two rates and Böhm-Bawerk's roundaboutness and made a
theory of the business cycle (*Prices and Production*, 1931). Credit
expansion holds the money rate below the natural rate. Entrepreneurs read
the low rate as a signal that people have saved more, so they lengthen the
structure of production: more mines and machine tools, more stages between
raw material and consumer. But no one has saved more. When the credit stops
or consumers bid resources back, the long projects can't be finished; the
boom's investments turn out to be *malinvestments*, and the bust is their
liquidation.

The interest rate, in this view, is a price like any other: a signal that
coordinates plans across time, between those who save and those who
invest. Section 8 cites Hayek's "Competition as a discovery procedure":
profit is the signal that capital is wanted somewhere. The rate of interest
is the same kind of signal about *when* it is wanted. Falsify it, and the
whole structure of production is built for a future no one chose.

Two arguments followed, and they are the heart of the question:

- **Sraffa (1932).** Piero Sraffa reviewed *Prices and Production* in the
  *Economic Journal* and asked: natural rate of *what*? In an economy
  without money, each commodity has its own rate of interest (how much
  wheat next year for wheat today, how much cloth for cloth), and when
  relative prices are changing, these "own rates" differ. So there is no
  single natural rate for the money rate to deviate from. Keynes built the
  own-rates idea into chapter 17 of the *General Theory*. The Austrian
  answer (Ludwig Lachmann's, later) was that arbitrage by entrepreneurs
  makes the own rates equal once each is measured in one money; the
  argument was never quite settled.
- **Knight (1930s).** Frank Knight denied there was a period of production
  at all. Capital is a permanent, self-renewing fund, like a lake with
  water flowing through it; it has no "length", and interest is simply its
  yield. Hayek's answer, *The Pure Theory of Capital* (1941), gave up the
  average period of production but not the point: capital is not a
  homogeneous fund but a *structure* of heterogeneous goods, each fitted to
  others in particular plans, and their value depends on those plans
  working out.

Mises (*Human Action*, 1949, chapter XIX) gives the decomposition the data
below can test: the market rate of interest is *originary interest* (pure
time preference, the discount of future against present satisfaction),
plus an *entrepreneurial component* (the lender's share of the venture's
risk), plus a *price premium* (expected changes in the purchasing power of
money).

## Fisher, Keynes, and Cambridge

- **Fisher** (*The Theory of Interest*, 1930) explained interest by
  impatience on one side and investment opportunity on the other; his
  earlier *Appreciation and Interest* (1896) gave the identity everyone now
  uses: nominal rate ≈ real rate + expected inflation. That is Mises's
  price premium, measured.
- **Keynes** (*General Theory*, 1936, chapters 13–17) made interest the
  reward not for saving but for parting with liquidity. It is set by the
  demand to hold money against the supply of it; it need not move to match
  saving and investment, and a rate too high for full employment can
  persist. Investment happens when the *marginal efficiency of capital*
  (Wicksell's natural rate, roughly) exceeds it.
- **The Cambridge controversy** (Joan Robinson, "The Production Function
  and the Theory of Capital", 1953–54; Sraffa, *Production of Commodities
  by Means of Commodities*, 1960) asked the question Hayek and Knight had
  argued over in another form: in what units is "capital" measured? Machines
  are different things; to add them you value them, and their values depend
  on the rate of profit. So the rate of profit can't be explained as the
  marginal product of a quantity of capital that is itself measured with
  the rate of profit. Worse, a technique abandoned as $`r`$ rises can come
  back at a higher $`r`$ still ("reswitching"). Samuelson conceded the logic
  ("A Summing Up", 1966).

Hayek and the Cambridge critics, opposite in almost everything, agree on
this: capital is not a homogeneous lump with a marginal product, and
$`r`$ is not a fact of technology. Something else sets it: time
preference, liquidity, convention, banks, the distribution of income. And
the entrepreneur reading it.

## The modern answer: r as an opportunity cost

Finance sidesteps the question of what capital *is*. For a shareholder, $`r`$
is what the same money could earn elsewhere at the same risk, and it is
built up in layers from the rate of interest:

```math
r \;=\; \underbrace{\rho}_{\text{real rate}}
\;+\; \underbrace{\pi^e}_{\text{expected inflation}}
\;+\; \underbrace{\tau}_{\text{term premium}}
\;+\; \underbrace{\sigma}_{\text{credit spread}}
\;+\; \underbrace{\epsilon}_{\text{equity premium}}
```

The real rate is Böhm-Bawerk's and Fisher's price of time (Mises's
originary interest); expected inflation, Fisher's and Mises's price premium;
the term premium, the extra for lending long rather than rolling short
loans; the credit spread, the risk the borrower doesn't pay; the equity
premium, the extra for owning the residual claim instead of a fixed one
(Mises's entrepreneurial component covers the last two). The capital asset
pricing model (Sharpe, 1964) scales the equity premium by the stock's beta.

The first four layers can be read off bond prices. The fifth can't. Mehra
and Prescott (1985) found that stocks had out-earned short-term
government debt by about six points a year, 1889–1978 (a real 7.0%
against 0.8%), far more than any reasonable aversion to risk explains: the
*equity premium puzzle*. In this repository's residual income work,
Berkshire's `--premium 0.04` is the equity premium, on top of the 10-year
yield, which already holds the first three layers (a company's equity has no
credit spread of its own: its risk is all in the premium).

## The data: how the rate has formed, 1857–2025

From FRED (`fred-fetch --list fred/rates.txt`; the series are listed in
[fred/rates.txt](../fred/rates.txt)), with
[fred/examples/rates.rs](../fred/examples/rates.rs):

    cargo run --release -p fred --example rates

Decade means, percent a year. *Short* is New York commercial paper to 1933
and the 3-month Treasury bill after; *long* is long-term government bonds
from 1925 (the 10-year from 1953); *high grade* is railroad bonds to 1918
and Moody's Aaa after; inflation is the NBER's general price level to 1913
and the CPI after. *Real* is nominal minus inflation as it turned out, not
as lenders expected it.

| Decade | Short | Inflation | Real short | Long | Term spread | High grade | Real high grade | Baa − Aaa | TIPS (10y) | Breakeven | Term premium |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1860s | 7.1 | | | | | 5.9 | | | | | |
| 1870s | 6.5 | −3.6 | 10.0 | | | 5.7 | 9.2 | | | | |
| 1880s | 5.1 | 0.0 | 5.1 | | | 4.0 | 4.0 | | | | |
| 1890s | 4.6 | 0.0 | 4.6 | | | 3.5 | 3.5 | | | | |
| 1900s | 4.8 | 2.1 | 2.7 | | | 3.5 | 1.4 | | | | |
| 1910s | 4.7 | 6.4 | −1.7 | | | 4.2 | −2.1 | 1.8 | | | |
| 1920s | 5.0 | −0.1 | 5.1 | 3.6 | −1.0 | 5.1 | 5.2 | 1.6 | | | |
| 1930s | 1.2 | −2.1 | 3.2 | 3.0 | 1.8 | 3.9 | 6.0 | 2.4 | | | |
| 1940s | 0.5 | 5.5 | −5.0 | 2.3 | 1.9 | 2.7 | −2.8 | 1.0 | | | |
| 1950s | 2.0 | 2.0 | 0.0 | 3.0 | 1.0 | 3.3 | 1.3 | 0.6 | | | |
| 1960s | 4.0 | 2.3 | 1.7 | 4.7 | 0.7 | 5.0 | 2.7 | 0.6 | | | |
| 1970s | 6.3 | 7.1 | −0.8 | 7.5 | 1.2 | 8.2 | 1.2 | 1.1 | | | |
| 1980s | 8.8 | 5.5 | 3.3 | 10.6 | 1.8 | 11.3 | 5.8 | 1.5 | | | |
| 1990s | 4.9 | 3.0 | 1.9 | 6.7 | 1.8 | 7.7 | 4.7 | 0.8 | | | 1.7 |
| 2000s | 2.7 | 2.6 | 0.1 | 4.5 | 1.8 | 6.0 | 3.4 | 1.1 | 2.0 | 2.2 | 1.0 |
| 2010s | 0.6 | 1.8 | −1.2 | 2.4 | 1.8 | 4.0 | 2.3 | 1.0 | 0.4 | 2.0 | 0.2 |
| 2020s | 2.8 | 3.9 | −1.2 | 3.0 | 0.2 | 4.1 | 0.2 | 0.9 | 0.7 | 2.2 | 0.1 |

(The 2020s are 2020–25. TIPS and breakeven inflation from 2003; the
term premium is the Kim–Wright model's, from 1990.)

### What it says

- **Under the gold standard, the market set a real rate of 4–5%.** From the
  1870s to the 1890s there was no central bank; prices fell, then were flat
  (the price level was 77 in 1879, 1889 and 1899), and commercial paper
  paid 4.6–6.5% while high-grade railroad bonds paid 3.5–5.7%. Whatever
  originary interest is, this is the closest the record comes to observing
  it without a monetary authority in the way. Even so, the real rate was
  falling decade by decade, 10% to 2.7%, as capital accumulated.
- **The real rate has been negative for whole decades four times:** the
  1910s and 1940s (war inflation; in the 1940s the Fed pegged government
  yields to finance the war), the 1970s (inflation outran a rising nominal
  rate), and 2010–25. Lenders were paid less than nothing for waiting. On
  Böhm-Bawerk's account that should not happen in a free market for long;
  on Wicksell's and Hayek's, it is what a money rate held below the natural
  rate looks like; on Keynes's and the secular-stagnation account (Summers,
  2014), the natural rate itself fell, and the money rate followed it.
  These data alone can't decide between them; the Laubach–Williams
  estimate of the natural rate (New York Fed) is the obvious next series.
- **Volcker's real rates (1980s, +3.3% short, +5.8% on Aaa)** matched the
  1930s', which were high for the opposite reason: prices fell (−2.1% a
  year) while nominal rates couldn't go below zero.
- **The 1920s curve was inverted:** commercial paper (5.0%) above long
  government bonds (3.6% from 1925), with the call-money boom of 1928–29
  pulling the short rate up. In every decade since, the long rate has been
  above the short one, by about 1–2 points.
- **The layers above the real rate are smaller and steadier.** The credit
  spread (Baa − Aaa) has averaged 1.2 points since 1919, peaking at 4.2 in
  1932. The 10-year term premium averaged 0.8 since 1990, and has fallen to
  about zero: in 2020 it was negative (−0.5). TIPS yields averaged 1.0%
  over 2003–25, from −0.9% (2021) to 2.3% (2006). The big swings in $`r`$
  come from the bottom layers, the real rate and inflation, not from risk.

### For the residual income model

A fixed $`r = 9\%`$ treats the cost of equity as constant when its base,
the 10-year yield, ran from 0.9% (2020) to 13.9% (1981).
[brk/README.md](brk/README.md#with-the-treasury-rate-as-r) redoes Berkshire
with the Treasury rate plus a premium; Apple's persistence analysis could
do the same.

But the theories also say what this repository can't measure: whether the
rate the market set was the *right* one. If Hayek is right about the 2010s,
low rates flattered every long-duration asset, and some of the excess
persistence measured against $`r`$ was the structure of production lengthening on borrowed time. If
Marx is right that surplus profit is competed away unless it is a rent,
then $`\omega`$ near 1 means a waterfall, and the question for Apple or
Berkshire is what theirs is.

## The equity premium, and the portfolio's two r's

The Optimates app measures each holding against MINT, an ultrashort bond
fund: "over r" or "under r" (Portfolio). That is one of two rates a
shareholder has a claim to call $`r`$, and the equity premium is most of
the distance between them.

### Two r's

The **rate on cash**, what money earns waiting: bills, or MINT, which is
bills plus a little short credit, less a fee.

```math
r_{\text{cash}} \;\approx\; \rho_{\text{short}} + \pi^e_{\text{short}} \;(+\,\sigma_{\text{short}} - \text{fee})
```

The **cost of equity**, what a share must be expected to earn to be worth
its price: the five layers above, built on the long yield.

```math
k \;=\; \underbrace{\rho + \pi^e + \tau}_{y_{10}} \;+\; \epsilon
```

The distance between them is the term premium (roughly; $`y_{10}`$ also
moves with the expected path of short rates) and the equity premium:

```math
k - r_{\text{cash}} \;\approx\; (y_{10} - r_{\text{bill}}) + \epsilon
```

On 25 November 2026: bills 4.37%, MINT's total return over the last twelve
months 4.20% (its fee and spread roughly cancel), the 10-year Treasury
4.92%, and the Kim–Wright term premium 0.89%. Cash is $`r_{\text{cash}}`$ ≈
4.2–4.4%; a share's $`k`$ is 4.9% plus $`\epsilon`$.

### Two equity premia

Measured against bills or against bonds:

```math
\text{ERP}_{\text{bill}} = \mathbb{E}[R_m] - r_{\text{bill}},
\qquad
\text{ERP}_{\text{bond}} = \mathbb{E}[R_m] - y_{10},
\qquad
\text{ERP}_{\text{bill}} \approx \text{ERP}_{\text{bond}} + \tau
```

The first is the CAPM's (Sharpe, 1964): a one-period model whose riskless
asset is a bill. The second is the valuer's (Damodaran's): a stock is a
long-duration claim, so it is discounted at a long rate. Mehra and Prescott's
six points (1889–1978) are the first kind.

### The premium implied by prices

Gordon's model (1959) prices a share as its next dividend over the
difference between its cost of equity and its growth, $`P = D_1/(k - g)`$,
so $`k = D_1/P + g`$. With retention $`b`$ and the return on reinvested
earnings equal to $`k`$ (Miller and Modigliani, 1961), growth is $`g = bk`$
and $`D_1 = (1-b)E_1`$, and the cost of equity is simply the earnings yield:

```math
k = \frac{(1-b)E_1}{P} + bk \;\;\Longrightarrow\;\; k = \frac{E_1}{P}
```

Earnings grow with the price level, so $`E/P`$ is a **real** return, and
the implied premium compares it with a real bond:

```math
\epsilon_{\text{real}} \;\approx\; \frac{E}{P} - y^{\text{TIPS}}_{10}
```

For the 39 large US companies in this repository's `market` table (27.8
trillion dollars of market value), the cap-weighted earnings yield on 24
November 2026 was 4.02%; the 10-year TIPS yield, 2.78% (October). The
implied real premium: about **1.2 points**, against the six of history. On
these companies (they are the largest, and grow faster than the assumption
$`\text{ROE} = k`$ allows, which understates them) the market prices in
little reward for owning the residual claim.

The **Fed model** compares $`E/P`$ with the *nominal* 10-year yield: 4.02%
against 4.92%, a premium of −0.9. Asness (2003) calls this an error: a real
yield set against a nominal one. It is the money illusion Modigliani and Cohn
(1979) said depressed stocks in the inflation of the 1970s.

### What "over r" measures

For a holding, the app reports the realized excess return over cash, in
dollars, on the money behind the shares still held, from each buy to today,
dividends on both sides:

```math
x_i \;=\; R_i - r_{\text{MINT}}
```

The CAPM's expectation of it is not zero but the holding's share of the
equity premium: $`\mathbb{E}[x_i] = \beta_i\,\text{ERP}_{\text{bill}}`$. A
stock with $`\beta = 1`$, held a year, is *expected* to be over r by
something like four to six percent of its cost, for bearing the market's
risk, without any skill at all. Skill is what's left over the market on the
same days, Jensen's alpha (1968):

```math
\alpha_i \;=\; x_i - \beta_i\,(R_m - r_{\text{MINT}})
```

So "over r" answers *did this money do better than cash?*, and "over
market" *did it do better than owning the market?* (the same dollars on the
same days in SPY, dividends included; $`\beta`$ taken as 1). Under r on
a stock means that, so far, the premium it was owed hasn't been paid. A
holding can be over r and under market at once: it beat cash by the equity
premium those days paid, but not the index that would have earned it for
nothing.

### Why MINT is the wrong r for a stock, and the right one for a portfolio

MINT's duration is about half a year; a stock's, at a 4% earnings yield,
something like $`1/(k-g)`$ ≈ 25 years. Measured against MINT, a stock's
excess return carries the term premium and every move in long rates as well
as the equity premium: in a year when long rates fall, long assets beat cash
whether or not their businesses did well. As a hurdle for *valuing* a
business, $`r_{\text{cash}}`$ is too low by $`\tau + \epsilon`$; the residual
income work uses $`k`$ on the 10-year.

But as the opportunity cost of the money actually in the account, MINT is
the honest one: it is where the money waits between uses, at the rate it
actually earned.

### How equities came to carry a premium

For most of the history of American securities the respectable holding was
the bond, above all the railroad bond; shares were for insiders and
speculators. *The Great Gatsby* (1925) catches the mood: Nick Carraway comes
East in 1922 to learn "the bond business", because "everybody I knew was in
the bond business".

The argument for shares came first from Edgar Lawrence Smith's *Common
Stocks as Long Term Investments* (1924), which set out to show that bonds
were the better investment and found the opposite, from 1866 on. His reason
was retained earnings: a company pays out part of its profit and reinvests
the rest, so its shares compound beyond their dividends, while a bond's
coupon is fixed. Keynes, reviewing it (1925), singled out exactly this, the
compound interest of retained earnings. The book became a text of the
1920s boom, and of Irving Fisher's "permanently high plateau" days before
the crash.

Graham and Dodd's *Security Analysis* (1934) was the reaction, not the
beginning: written after the crash, it asks for a margin of safety and
earnings power proven over years, and still treats a sound bond as the
default; Graham's *Intelligent Investor* (1949) keeps a quarter to three
quarters in stocks, never all. Equities were accepted after the war, in the
"cult of the equity" of the 1950s (George Ross Goobey moving the Imperial
Tobacco pension fund into shares), and the moment can be dated: in 1958–59
dividend yields fell below bond yields for the first time, the reverse
yield gap. Until then a share had to out-yield a bond to pay for its risk;
from then the market priced Smith's point, that a share's earnings grow and
a bond's coupons don't. Miller and Modigliani (1961) gave it its theory:
paying out earnings or retaining them doesn't change a company's value,
only the form of the return, which is why the residual income model above
can value a company from its book and earnings whatever its dividends.

Mehra and Prescott (1985) then measured what this had paid, about six
points a year over bills since 1889, and showed the standard model of
consumption could justify a third of a point; Weil (1989) added that the
same model, made to fit the premium, predicts a risk-free rate far above
the one observed.

### Why long bonds lag a money-market fund

The premium is what equities pay for the risk bonds don't carry, so bonds
are meant to earn less. Over shorter spans a bond fund's return is

    return ≈ the yield it started at − duration × the change in yields

From August 2024 to November 2026 the 30-year Treasury yield rose from
4.15% to 5.30%: a long corporate fund (duration 12–13 years) lost about 14%
in price over those two and a quarter years, against about 12% earned in
interest. A fund
of duration half a year, like MINT, barely feels a change in yields, and in
2022–24 the curve was inverted (the 3-month bill at 5.2% against the
30-year at 4.2%), so the longer bond also paid less. Over a holding period
about its duration, a bond fund's return converges to the yield it was
bought at, since the losses on price are reinvested at the higher yields;
and long bonds' place in a portfolio has been as a hedge, rising when
stocks fall, as they did from 2000 to 2021. When inflation moves both, as
in 2022, they fall together and the hedge is gone.

### The puzzle, and the tails

Mehra and Prescott's puzzle is that six points of premium need an aversion
to risk far beyond what people show in other choices. One answer is that the
premium pays for rare disasters (Rietz, 1988; Barro, 2006): a small chance
of losing half of everything, which a sample without such a disaster never
shows. Taleb's is that the tails are fat, so the sample average of a
premium is a poor estimate of its expectation (Taleb, 2020), and Peters
(2019) adds that an investor lives one path through time, not the average of
many: the premium that compounds for one shareholder is less than the
ensemble's, by about half the variance. On these readings the premium is not
a free lunch but rent for carrying ruin risk, and an implied premium of one
point is a market that has stopped charging much for it.

## Rates of return: IRR and XIRR

"Over r" in dollars says how much; a premium on r is a rate, and to compare
a position's return with r it must be a rate per year, like r itself.

### Holding-period return

One sum invested at time 0, worth V at time T (in years), with dividends D:

    R = (V + D − C) / C,        annualized: (1 + R)^(1/T) − 1

This is enough for one purchase. It fails as soon as money goes in at
different times: a position bought in three lots, over three years, has no
single T.

### IRR: the rate that prices all the flows at once

Write every flow as a signed amount c_t, money in negative and money out
(or held, at today's value) positive. The internal rate of return is the r
at which they are worth nothing net:

    NPV(r) = Σ_t c_t / (1 + r)^t = 0

With one flow in and one out it is the annualized holding-period return
above. With many, it is the single constant rate at which the money, each
dollar from the day it went in, would have grown into what came out. Since
the investor chose when to put money in, the IRR is *money-weighted*: a
lucky large purchase before a rise counts for more than a small one. That
is the right measure of what happened to the investor's dollars; the
*time-weighted* return (chaining each period's return, ignoring the size of
the flows) is the right measure of a manager who doesn't control them, and
is what an index or a fund reports.

### XIRR: irregular dates

IRR as usually written assumes evenly spaced periods. Real flows come on
any day, so XIRR (the spreadsheet name; the same idea) measures time in
years from the first flow:

    Σ_i c_i / (1 + r)^(t_i) = 0,        t_i = (d_i − d_0) / 365

There is no closed form. The function is solved numerically by Newton's
method,

    r_{n+1} = r_n − NPV(r_n) / NPV′(r_n),
    NPV′(r) = Σ_i −t_i c_i / (1 + r)^(t_i + 1)

starting from 10%. Newton's method converges fast near the root but can
shoot past it, so the solver (`xirr` in `equities/api/src/rpc.rs`) keeps a
bracket [lo, hi] where NPV changes sign, starting at (−99.99%, +1000%), and
bisects whenever a Newton step would leave it. Without a sign change (money
only going in, say) there is no rate, and none is reported. With flows that
change sign more than once there can be more than one root (Descartes' rule
of signs); a portfolio's flows (buys, then dividends and a final value) change
sign once, so the root is unique.

### The premium on r

For each position held a year or more the API solves three XIRRs, on the
same buy dates and amounts:

- **yours:** the buys in, the dividends out on the days they were paid, and
  today's value out;
- **r's:** the buys in, and what the same dollars on the same days would be
  worth in MINT today;
- **the market's:** the same, in SPY.
- **r10's:** the buys in, each compounding at the 10-year Treasury yield of
  its own day (DGS10) to today: what a ten-year note bought then and held
  would pay, with no price risk. Short r is the hurdle of cash; r10 is the
  long-run hurdle a stock, a long-lived claim, had to clear, and over it is
  the equity premium earned.

The premium on r is yours minus r's, in points per year; over the market,
yours minus the market's. Because the flows are the same, the comparison is
of the asset alone: the timing was the investor's in both. Accounts and the
whole portfolio pool their positions' flows and solve once, which weights
each position by its dollars and its time.

Rates for less than a year are not annualized, following the GIPS
standard: three weeks' 2% move compounds to 40% a year, which says
nothing. Those positions show their dollars only.

The limits: the baselines are total-return prices (dividends reinvested),
while the position's dividends are counted as cash out on the day paid, not
reinvested: equivalent for the IRR, which already assumes flows earn the
rate itself. A position's cost basis can include more than its buys (wash
sales, exercised options), so the buys are scaled up to the brokerage's cost:
the extra dollars are assumed to have gone in when the shares did. And an
IRR is not a forecast: it is the rate the past paid, on one path (Peters,
above).

## References

- Asness, C. S. (2003). Fight the Fed model. *Journal of Portfolio
  Management* 30(1).
- Barro, R. J. (2006). Rare disasters and asset markets in the twentieth
  century. *Quarterly Journal of Economics* 121(3).
- Böhm-Bawerk, E. von (1884, 1889). *Capital and Interest*; *The Positive
  Theory of Capital*.
- Damodaran, A. Equity risk premiums: determinants, estimation and
  implications (updated annually). NYU Stern.
- Fitzgerald, F. S. (1925). *The Great Gatsby*. Scribner.
- Fisher, I. (1896). *Appreciation and Interest*. (1930). *The Theory of
  Interest*.
- Gordon, M. J. (1959). Dividends, earnings, and stock prices. *Review of
  Economics and Statistics* 41(2).
- Graham, B. (1949). *The Intelligent Investor*. Harper.
- Graham, B., and D. L. Dodd (1934). *Security Analysis*. Whittlesey House.
- Hayek, F. A. (1931). *Prices and Production*. (1941). *The Pure Theory
  of Capital*.
- Jensen, M. C. (1968). The performance of mutual funds in the period
  1945–1964. *Journal of Finance* 23(2).
- Keynes, J. M. (1925). An American study of shares versus bonds as
  permanent investments. *The Nation and Athenaeum*, 2 May.
- Keynes, J. M. (1936). *The General Theory of Employment, Interest and
  Money*, chapters 13–17.
- Kim, D. H., and J. H. Wright (2005). An arbitrage-free three-factor term
  structure model and the recent behavior of long-term yields and
  distant-horizon forward rates. Federal Reserve Board, FEDS 2005-33.
- Laubach, T., and J. C. Williams (2003). Measuring the natural rate of
  interest. *Review of Economics and Statistics* 85(4).
- Marx, K. (1867, 1894). *Capital*, volumes I and III (Parts II, V, VI).
- Mehra, R., and E. C. Prescott (1985). The equity premium: a puzzle.
  *Journal of Monetary Economics* 15(2).
- Miller, M. H., and F. Modigliani (1961). Dividend policy, growth, and
  the valuation of shares. *Journal of Business* 34(4).
- Mises, L. von (1949). *Human Action*, chapter XIX.
- Modigliani, F., and R. A. Cohn (1979). Inflation, rational valuation
  and the market. *Financial Analysts Journal* 35(2).
- Peters, O. (2019). The ergodicity problem in economics. *Nature
  Physics* 15.
- Rietz, T. A. (1988). The equity risk premium: a solution. *Journal of
  Monetary Economics* 22(1).
- Robinson, J. (1953–54). The production function and the theory of
  capital. *Review of Economic Studies* 21(2).
- Samuelson, P. A. (1966). A summing up. *Quarterly Journal of Economics*
  80(4).
- Sharpe, W. F. (1964). Capital asset prices: a theory of market
  equilibrium under conditions of risk. *Journal of Finance* 19(3).
- Sraffa, P. (1932). Dr. Hayek on money and capital. *Economic Journal*
  42(165). (1960). *Production of Commodities by Means of Commodities*.
- Smith, E. L. (1924). *Common Stocks as Long Term Investments*. Macmillan.
- Summers, L. H. (2014). U.S. economic prospects: secular stagnation,
  hysteresis, and the zero lower bound. *Business Economics* 49(2).
- Taleb, N. N. (2020). *Statistical Consequences of Fat Tails*. STEM
  Academic Press.
- Weil, P. (1989). The equity premium puzzle and the risk-free rate
  puzzle. *Journal of Monetary Economics* 24(3).
- Wicksell, K. (1898). *Interest and Prices*.

Data: FRED, Federal Reserve Bank of St. Louis; series as listed in
`fred/rates.txt`, from the Federal Reserve Board (H.15), the NBER
Macrohistory Database, Moody's, the Bureau of Labor Statistics, the
Cleveland Fed and the OECD. Not endorsed or certified by the Federal
Reserve Bank of St. Louis.
