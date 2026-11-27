# Taiwanese equities

Notes from 2026-11-22, after TSMC's charts came up thin. Like
`name/JAPANESE.md` and EDINET, a sketch of where a Taiwanese source would
come from. Hanzi here are traditional characters (正體字, *zhèngtǐzì*, as
Taiwan writes them), with Hanyu Pinyin, Taiwan's official romanization since
2009 (older names keep Wade–Giles: Taipei is *Táiběi*, Hsinchu *Xīnzhú*).
What I haven't checked yet is marked so.

## Why TSMC is thin

TSMC (台灣積體電路製造, *Táiwān Jītǐ Diànlù Zhìzào*, "Taiwan Integrated
Circuit Manufacturing") trades in Taipei as **2330** and in New York as
**TSM** (one ADR is five shares). On EDGAR it is a foreign private issuer:

| | |
|---|---|
| Statements in XBRL | 9 annual reports (20-F), filed 2018–2026 |
| Values | 2015–2025, in TWD, and a USD convenience translation |
| Quarterly | none: quarterly results come as 6-K press releases, without XBRL statements |

So its quarterly charts are empty, and the dashboard's "auto" period shows
years for it (store migration 006, `flow_period`). The same holds for every
company that files 20-Fs: Toyota's TM, the Nikkei list's ADRs, Ferrari.

## The source: MOPS

**MOPS**, the Market Observation Post System (公開資訊觀測站, *Gōngkāi
Zīxùn Guāncè Zhàn*, "public information observation station"), is Taiwan's
EDGAR or EDINET: every listed company's filings in one place, run by the
Taiwan Stock Exchange.

- **Regulator**: the Financial Supervisory Commission (金融監督管理委員會,
  *Jīnróng Jiāndū Guǎnlǐ Wěiyuánhuì*), through its Securities and Futures
  Bureau (證券期貨局, *Zhèngquàn Qīhuò Jú*).
- **Markets**: the Taiwan Stock Exchange (臺灣證券交易所, *Táiwān Zhèngquàn
  Jiāoyìsuǒ*, TWSE: the main board), the Taipei Exchange (證券櫃檯買賣中心,
  *Zhèngquàn Guìtái Mǎimài Zhōngxīn*, TPEx: over the counter), and an
  emerging board.
- **Reports**: quarterly (季報, *jìbào*: Q1, half year, Q3, reviewed by the
  auditor) and annual (年報, *niánbào*, audited). As with EDINET and the
  SEC, quarterly cash flows are year to date.
- **Accounting**: IFRS as endorsed by the FSC ("Taiwan-IFRSs"), mandatory for
  listed companies since 2013, with financial statements filed in XBRL. So
  TSMC would have quarterly figures from 2013, against annual ones from 2015
  through EDGAR. (The 2013 start for XBRL is my understanding; to confirm.)
- **The statements**:
  - 資產負債表 (*zīchǎn fùzhài biǎo*), the balance sheet;
  - 綜合損益表 (*zònghé sǔnyì biǎo*), the statement of comprehensive income;
  - 現金流量表 (*xiànjīn liúliàng biǎo*), the cash flow statement;
  - 權益變動表 (*quányì biàndòng biǎo*), changes in equity.
- **Dates in the Republic of China calendar** (民國紀年, *Mínguó jìnián*):
  year 1 is 1912, so ROC 113 is 2024 (add 1911). MOPS's forms and many of its
  files use it; a reader has to convert.

### Getting at it (to check before building)

- **MOPS itself** serves its pages from web forms (HTML, form posts), and
  offers XBRL instance documents per company and quarter, with bulk
  downloads by quarter, I believe. How stable and scriptable those are
  decides how hard this is.
- **TWSE's OpenAPI** (openapi.twse.com.tw) publishes JSON, including summary
  income statements and balance sheets, but mainly for the latest period,
  not the history.
- **Prices**: TWSE publishes daily prices per stock (JSON, by month); Tiingo
  is US-centered and may not have Taipei listings. (To check.)

## Who's there

The TAIEX (臺灣加權股價指數, *Táiwān Jiāquán Gǔjià Zhǐshù*, the
capitalization-weighted index) is dominated by TSMC, a third or more of it.
Some of the large names, with the US ADRs where they have them:

| Code | Company | ADR |
|---|---|---|
| 2330 | 台積電 (*Táijīdiàn*), TSMC | TSM |
| 2317 | 鴻海 (*Hónghǎi*), Hon Hai, "Foxconn" | |
| 2454 | 聯發科 (*Liánfākē*), MediaTek | |
| 2308 | 台達電 (*Táidádiàn*), Delta Electronics | |
| 2303 | 聯電 (*Liándiàn*), United Microelectronics | UMC |
| 3711 | 日月光投控 (*Rìyuèguāng Tóukòng*), ASE Technology | ASX |
| 2412 | 中華電信 (*Zhōnghuá Diànxìn*), Chunghwa Telecom | CHT |

## The plan

A `mops` crate beside `edgar` and `edinet`, the same shape: a cache of
filings as downloaded, a client that fetches what's missing (politely; MOPS
has no published rate limit that I know of), and a reader in `equities`
mapping the Chinese line items to `item` (with pinyin, as the Japanese
mapping has Hepburn). Tickers as `2330.TW`, like `7203.T`; prices from TWSE.
Before any of it, the access questions above.
