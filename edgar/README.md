# edgar

SEC EDGAR as a source: companies' filing lists, and the statement pages of
their 10-Q, 10-K and 20-F filings, kept on disk under `cache/`. `Cache`
reads it and never touches the network; with the `fetch` feature, `Client`
downloads into it (under 10 requests a second, as the SEC asks).

```text
cache/company_tickers.json                ticker -> CIK
cache/<cik>/submissions.json              the company's filings
cache/<cik>/CIK…-submissions-001.json     older filings, paged out
cache/<cik>/<accession>/FilingSummary.xml the filing's index of pages
cache/<cik>/<accession>/R2.htm …          its statement pages
```

## A filing's pages: FilingSummary.xml and R1.htm, R2.htm, …

A company files its statements twice over: as the document people read, and
as XBRL, the same figures tagged with standard elements (`us-gaap_Revenues`)
or its own (`tsla_…`). EDGAR renders the XBRL as one HTML page per statement
and note, `R1.htm`, `R2.htm`, … (what the filing's "Interactive Data" viewer
shows), and indexes them in `FilingSummary.xml`: each page's file, title and
kind. Tesla's 10-Q for the third quarter of 2025, for example:

| Page | Title |
|---|---|
| R1 | Document - Cover (shares outstanding, the period) |
| R2 | Statement - Consolidated Balance Sheets |
| R3 | Statement - Consolidated Balance Sheets (Parenthetical) |
| R4 | Statement - Consolidated Statements of Operations |
| R5–R7 | comprehensive income; equity |
| R8 | Statement - Consolidated Statements of Cash Flows |
| R9 … | Disclosure - the notes |

The numbers differ from filing to filing, so the reader (`equities`'
`filing` module) never assumes R2 is the balance sheet: it reads
`FilingSummary.xml` and opens the pages whose titles are statements. The
pages suit it because every row carries its XBRL element in its markup,
which is how a row becomes an item (Revenue, StockholdersEquity) whatever a
company calls the line. Each row is wrapped in a good deal of HTML, so a
statement page runs to 100–150 KB.

## What is in git, and what isn't

`cache/` is not (it is gitignored, and holds every company that has filed
since 2009). A filing never changes once filed: a correction is a new filing
with its own accession number, so an accession's pages are fixed, and new
reports only add folders.

The few filings in git are test inputs, beside the tests that read them, and
only the pages those read: `equities/tsla/filings/<accession>/` (four 10-Qs:
FilingSummary.xml, R2, R4 and sometimes R8), and one annual report each in
`equities/gs/filings/`, `f/filings/`, `gm/filings/` and `tm/filings/`
(the statements each test reads, 300-700 KB a filing). Their tests
check every line the reader reads against the statements transcribed by
hand, and with the pages in git they run anywhere, a fresh checkout or a
cloud build, without the SEC.

## Executables

- `fetch` (the `fetch` feature): filings into `cache/`.
- `universe`: every company that filed a 10-K, 10-Q or 20-F since a date,
  by CIK, from EDGAR's bulk submissions.zip.
- `changed`: which companies got filings since a time, from the cache alone.
- `edgar-progress`: how far a fetch of every company has got.
- `edgar-funds`: the fund data sets (N-CEN, N-PORT).

The nightly job is `nightly/all.sh` (launchd, `pub.atec.edgar.all`).
